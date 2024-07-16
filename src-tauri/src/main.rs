// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use btleplug::api::bleuuid::uuid_from_u16;
use btleplug::api::CentralEvent::{
    DeviceConnected, DeviceDisconnected, DeviceDiscovered, DeviceUpdated,
    ManufacturerDataAdvertisement, ServiceDataAdvertisement, ServicesAdvertisement,
};
use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager as BtleManager, Peripheral};
use futures::StreamExt;
use tauri::{AppHandle, Manager, State};
use tokio;
use tokio::sync::Mutex;

#[derive(serde::Serialize, Clone)]
struct BleDevice {
    name: String,
    address: String,
}

struct BleConnection<'a> {
    central: Mutex<Option<Adapter>>,
    peripheral: &'a Mutex<Option<Peripheral>>,
    scan_devices: Mutex<Vec<BleDevice>>,
}

#[tauri::command]
async fn request_central_events<'a>(
    app_handle: AppHandle,
    connection: State<'a, BleConnection<'a>>,
) -> Result<bool, String> {
    let central = connection.central.lock().await;
    let peripheral = connection.peripheral.lock().await;
    let central = central.as_ref().unwrap();
    let central = central.clone();

    let mut event_stream = central.events().await.unwrap();

    tauri::async_runtime::spawn(async move {
        while let Some(event) = event_stream.next().await {
            if let DeviceDiscovered(_) | DeviceUpdated(_) = event.clone() {
                let mut devices = vec![];
                for p in central.peripherals().await.unwrap() {
                    devices.push(BleDevice {
                        name: p
                            .properties()
                            .await
                            .unwrap()
                            .unwrap()
                            .local_name
                            .unwrap_or("Unknown".to_string()),
                        address: p.address().to_string(),
                    });
                }
                println!("Devices: {:?}", serde_json::to_string(&devices).unwrap());
                app_handle
                    .emit_all("scan-list-update", serde_json::to_string(&devices).unwrap())
                    .unwrap();
            }
            if let DeviceConnected(_) = event.clone() {
                let peripheral = peripheral.as_ref().unwrap().clone();
                app_handle
                    .emit_all(
                        "device-connected",
                        serde_json::to_string(&BleDevice {
                            name: peripheral
                                .properties()
                                .await
                                .unwrap()
                                .unwrap()
                                .local_name
                                .unwrap_or("Unknown".to_string()),
                            address: peripheral.address().to_string(),
                        })
                        .unwrap(),
                    )
                    .unwrap();
            }
        }
    });

    Ok(true)
}
#[tauri::command]
async fn start_scan(connection: State<'_, BleConnection>) -> Result<bool, String> {
    let central = connection.central.lock().await;
    let central = central.as_ref().unwrap();
    central
        .start_scan(ScanFilter::default())
        .await
        .unwrap_or_else(|_| {
            println!("Failed to start scan");
        });
    Ok(true)
}

#[tauri::command]
async fn stop_scan(connection: State<'_, BleConnection>) -> Result<bool, String> {
    let central = connection.central.lock().await;
    let central = central.as_ref().unwrap();
    central.stop_scan().await.unwrap_or_else(|_| {
        println!("Failed to stop scan");
    });
    Ok(true)
}

#[tauri::command]
async fn connect(
    address: String,
    connection: State<'_, BleConnection>,
    app_handle: AppHandle,
) -> Result<bool, String> {
    let central = connection.central.lock().await;
    let central = central.as_ref().unwrap();

    let peripheral = central
        .peripherals()
        .await
        .unwrap()
        .into_iter()
        .find(|p| p.address().to_string() == address)
        .unwrap()
        .clone();
    peripheral.connect().await.unwrap();

    *connection.peripheral.lock().await = Some(peripheral.clone());

    peripheral.discover_services().await.unwrap_or_else(|_| {
        println!("Failed to discover services");
    });
    let service = peripheral
        .services()
        .into_iter()
        .find(|s| s.uuid == uuid_from_u16(0x180D))
        .unwrap();
    let characteristic = service
        .characteristics
        .into_iter()
        .find(|c| c.uuid == uuid_from_u16(0x2A37))
        .unwrap();

    peripheral
        .subscribe(&characteristic)
        .await
        .unwrap_or_else(|_| {
            println!("Failed to subscribe to characteristic");
        });

    tokio::spawn(async move {
        let mut notification_stream = peripheral.notifications().await.unwrap();
        while let Some(notification) = notification_stream.next().await {
            if notification.uuid == uuid_from_u16(0x2A37) {
                app_handle
                    .emit_all("heart-rate", notification.value[1])
                    .unwrap();
            }
        }
    });

    Ok(true)
}

#[tauri::command]
async fn disconnect(connection: State<'_, BleConnection>) -> Result<bool, String> {
    let connection = connection.peripheral.lock().await;
    let connection = connection.as_ref().unwrap();

    connection.disconnect().await.unwrap();

    Ok(true)
}

#[tauri::command]
async fn get_connected_device(connection: State<'_, BleConnection>) -> Result<String, String> {
    let connection = connection.peripheral.lock().await;
    let connection = connection.as_ref().unwrap();

    Ok(serde_json::to_string(&BleDevice {
        name: connection
            .properties()
            .await
            .unwrap()
            .unwrap()
            .local_name
            .unwrap_or("Unknown".to_string()),
        address: connection.address().to_string(),
    })
    .unwrap())
}

#[tokio::main]
async fn main() {
    let ble_manager = BtleManager::new().await.unwrap();
    let central = ble_manager
        .adapters()
        .await
        .unwrap()
        .into_iter()
        .next()
        .unwrap();

    tauri::Builder::default()
        .manage(BleConnection {
            central: Mutex::new(Some(central)),
            peripheral: Default::default(),
            scan_devices: Default::default(),
        })
        .invoke_handler(tauri::generate_handler![
            request_central_events,
            start_scan,
            stop_scan,
            connect,
            disconnect,
            get_connected_device
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
