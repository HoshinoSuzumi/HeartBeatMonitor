// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use btleplug::api::bleuuid::uuid_from_u16;
use btleplug::api::CentralEvent::{DeviceDisconnected, DeviceDiscovered, DeviceUpdated};
use btleplug::api::{BDAddr, Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager as BtleManager, Peripheral, PeripheralId};
use futures::StreamExt;
use std::error::Error;
use std::sync::Arc;
use tauri::{AppHandle, Manager, State};
use tokio;
use tokio::sync::Mutex;

#[derive(serde::Serialize, Clone)]
struct BleDevice {
    name: String,
    address: BDAddr,
    rssi: i16,
}

struct BleConnection {
    _is_events_registered: Arc<Mutex<bool>>,
    central: Arc<Mutex<Option<Adapter>>>,
    peripheral: Arc<Mutex<Option<Peripheral>>>,
}

impl BleConnection {
    fn new(central: Adapter) -> Self {
        Self {
            _is_events_registered: Arc::new(Mutex::new(false)),
            central: Arc::new(Mutex::new(Some(central))),
            peripheral: Arc::new(Mutex::new(None)),
        }
    }

    async fn set_peripheral(&self, peripheral: Option<Peripheral>) {
        let mut p = self.peripheral.lock().await;
        *p = peripheral;
    }

    pub async fn start_scan(&self) -> Result<(), String> {
        let central = self.central.lock().await;
        if let Err(e) = central
            .as_ref()
            .unwrap()
            .start_scan(ScanFilter { services: vec![] })
            .await
        {
            return Err(e.to_string());
        } else {
            Ok(())
        }
    }

    pub async fn stop_scan(&self) -> Result<(), String> {
        let central = self.central.lock().await;
        if let Err(e) = central.as_ref().unwrap().stop_scan().await {
            return Err(e.to_string());
        } else {
            Ok(())
        }
    }

    pub async fn is_connected(&self) -> bool {
        let peripheral = self.peripheral.lock().await;
        peripheral.is_some()
    }

    pub async fn connect(&self, address: BDAddr, app: &AppHandle) -> Result<(), Box<dyn Error>> {
        self.stop_scan().await.unwrap();
        let central = self.central.lock().await;
        let peripheral = central
            .as_ref()
            .unwrap()
            .peripheral(&PeripheralId::from(address))
            .await?;
        peripheral.connect().await?;
        peripheral.discover_services().await?;
        // 如果 peripheral.services() 不包含 0x180D 服务，则返回错误
        if !peripheral
            .services()
            .iter()
            .any(|s| s.uuid == uuid_from_u16(0x180D))
        {
            return Err("Peripheral does not have the required service".into());
        }

        self.set_peripheral(Some(peripheral)).await;

        let peripheral = self.peripheral.lock().await;
        let device = BleDevice {
            name: peripheral
                .as_ref()
                .unwrap()
                .properties()
                .await?
                .unwrap()
                .local_name
                .unwrap_or("Unknown".to_string()),
            address: peripheral.as_ref().unwrap().address(),
            rssi: peripheral
                .as_ref()
                .unwrap()
                .properties()
                .await?
                .unwrap()
                .rssi
                .unwrap(),
        };

        let service = peripheral
            .as_ref()
            .unwrap()
            .services()
            .into_iter()
            .find(|s| s.uuid == uuid_from_u16(0x180D))
            .unwrap();
        let characteristic = service
            .characteristics
            .into_iter()
            .find(|c| c.uuid == uuid_from_u16(0x2A37))
            .unwrap();

        let peripheral = peripheral.clone();
        peripheral
            .as_ref()
            .unwrap()
            .subscribe(&characteristic)
            .await?;
        let app_clone = app.clone();

        tokio::spawn(async move {
            let mut notification_stream =
                peripheral.as_ref().unwrap().notifications().await.unwrap();
            while let Some(notification) = notification_stream.next().await {
                if notification.uuid == uuid_from_u16(0x2A37) {
                    let value = notification.value;
                    let heart_rate = value[1] as u16;
                    app_clone.emit_all("heart-rate", heart_rate).unwrap();
                }
            }
        });

        app.emit_all("device-connected", device).unwrap();
        Ok(())
    }

    pub async fn disconnect(&self) -> Result<(), Box<dyn Error>> {
        let mut peripheral = self.peripheral.lock().await;
        peripheral.as_ref().unwrap().disconnect().await?;
        *peripheral = None;
        Ok(())
    }

    pub async fn register_central_events(&self, app: &AppHandle) {
        let central = self.central.lock().await;
        let central_clone = central.clone(); // Clone the central variable
        let app_handle = app.clone(); // Clone the AppHandle to move into the tokio::spawn closure
        let mut event_stream = central.as_ref().unwrap().events().await.unwrap();

        let self_clone = self.clone(); // Clone the BleConnection to move into the tokio::spawn closure

        tokio::spawn(async move {
            while let Some(event) = event_stream.next().await {
                match event {
                    DeviceDiscovered(peripheral) | DeviceUpdated(peripheral) => {
                        let p = central_clone
                            .as_ref()
                            .unwrap()
                            .peripheral(&peripheral)
                            .await
                            .unwrap();
                        let device = BleDevice {
                            name: p
                                .properties()
                                .await
                                .unwrap()
                                .unwrap()
                                .local_name
                                .unwrap_or("Unknown".to_string()),
                            address: p.address(),
                            rssi: p.properties().await.unwrap().unwrap().rssi.unwrap(),
                        };
                        app_handle
                            .emit_all("device-discovered", Some(device))
                            .unwrap();
                    }
                    DeviceDisconnected(peripheral) => {
                        let mut p = self_clone.peripheral.lock().await;
                        if let Some(peri) = p.as_ref() {
                            if peri.id() == peripheral {
                                app_handle
                                    .emit_all("device-disconnected", peripheral.to_string())
                                    .unwrap();
                                *p = None;
                            }
                        }
                    }
                    _ => {}
                }
            }
        });
    }
    // implements a Clone
    pub fn clone(&self) -> Self {
        Self {
            _is_events_registered: self._is_events_registered.clone(),
            central: self.central.clone(),
            peripheral: self.peripheral.clone(),
        }
    }
}

#[tauri::command]
async fn register_central_events<'a>(
    app_handle: AppHandle,
    connection: State<'a, BleConnection>,
) -> Result<bool, String> {
    if *connection._is_events_registered.lock().await {
        return Ok(false);
    } else {
        connection.register_central_events(&app_handle).await;
        *connection._is_events_registered.lock().await = true;
        Ok(true)
    }
}

#[tauri::command]
async fn start_scan(connection: State<'_, BleConnection>) -> Result<bool, String> {
    let err = connection.start_scan().await;
    if let Err(e) = err {
        return Err(e.to_string());
    } else {
        Ok(true)
    }
}

#[tauri::command]
async fn stop_scan(connection: State<'_, BleConnection>) -> Result<bool, String> {
    let err = connection.stop_scan().await;
    if let Err(e) = err {
        return Err(e.to_string());
    } else {
        Ok(true)
    }
}

#[tauri::command]
async fn is_connected(connection: State<'_, BleConnection>) -> Result<bool, String> {
    let status = connection.is_connected().await;
    Ok(status)
}

#[tauri::command]
async fn get_connected_device(connection: State<'_, BleConnection>) -> Result<BleDevice, String> {
    let peripheral = connection.peripheral.lock().await;
    let device = BleDevice {
        name: peripheral
            .as_ref()
            .unwrap()
            .properties()
            .await
            .unwrap()
            .unwrap()
            .local_name
            .unwrap_or("Unknown".to_string()),
        address: peripheral.as_ref().unwrap().address(),
        rssi: peripheral
            .as_ref()
            .unwrap()
            .properties()
            .await
            .unwrap()
            .unwrap()
            .rssi
            .unwrap(),
    };
    Ok(device)
}

#[tauri::command]
async fn connect(
    address: BDAddr,
    connection: State<'_, BleConnection>,
    app_handle: AppHandle,
) -> Result<bool, String> {
    if let Err(e) = connection.connect(address, &app_handle).await {
        Err(e.to_string())
    } else {
        Ok(true)
    }
}

#[tauri::command]
async fn disconnect(connection: State<'_, BleConnection>) -> Result<bool, String> {
    if let Err(e) = connection.disconnect().await {
        Err(e.to_string())
    } else {
        Ok(true)
    }
}

#[tokio::main]
async fn main() {
    let ble_manager = BtleManager::new().await.unwrap();
    let central = ble_manager
        .adapters()
        .await
        .unwrap()
        .into_iter()
        .nth(0)
        .unwrap();

    tauri::Builder::default()
        .manage(BleConnection::new(central))
        .invoke_handler(tauri::generate_handler![
            register_central_events,
            start_scan,
            stop_scan,
            is_connected,
            connect,
            disconnect,
            get_connected_device
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
