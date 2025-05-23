// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use btleplug::api::bleuuid::uuid_from_u16;
use btleplug::api::CentralEvent::{DeviceDisconnected, DeviceDiscovered, DeviceUpdated};
use btleplug::api::{BDAddr, Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager as BtleManager, Peripheral};
use futures::StreamExt;
use std::error::Error;
use std::sync::Arc;
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Emitter, Manager, State, WebviewUrl, WebviewWindowBuilder};
use tokio;
use tokio::sync::Mutex;
use warp::Filter;

#[derive(serde::Serialize, Clone, Debug)]
struct BleDevice {
    peripheral_id: String,
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

    pub async fn connect(
        &self,
        peripheral_id: String,
        app: &AppHandle,
    ) -> Result<(), Box<dyn Error>> {
        self.stop_scan().await.unwrap();
        let central = self.central.lock().await;
        let peripheral = central
            .as_ref()
            .unwrap()
            .peripherals()
            .await?
            .into_iter()
            .find(|p| p.id().to_string() == peripheral_id)
            .ok_or_else(|| "5010")?;
        peripheral.connect().await?;
        peripheral.discover_services().await?;
        // 如果 peripheral.services() 不包含 0x180D 服务，则返回错误
        if !peripheral
            .services()
            .iter()
            .any(|s| s.uuid == uuid_from_u16(0x180D))
        {
            return Err("5011".into());
        }

        self.set_peripheral(Some(peripheral)).await;

        let peripheral = self.peripheral.lock().await;
        let device = BleDevice {
            peripheral_id: peripheral.as_ref().unwrap().id().to_string(),
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
                .unwrap_or(0),
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
                    println!("Received notification: {:?}", value);
                    if value.len() < 2 {
                        continue;
                    }
                    let heart_rate = value[1];
                    app_clone.emit("heart-rate", heart_rate).unwrap();
                }
            }
        });

        app.emit("device-connected", device).unwrap();
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
                    DeviceDiscovered(peripheral_id) | DeviceUpdated(peripheral_id) => {
                        let p = central_clone
                            .as_ref()
                            .unwrap()
                            .peripheral(&peripheral_id)
                            .await
                            .unwrap();
                        if let Ok(Some(props)) = p.properties().await {
                            let name = props.local_name.unwrap_or("Unknown".to_string());
                            let rssi = props.rssi.unwrap_or(0);
                            let device = BleDevice {
                                peripheral_id: peripheral_id.to_string(),
                                name,
                                address: props.address,
                                rssi,
                            };
                            let _ = app_handle.emit("device-discovered", Some(device));
                        }
                    }
                    DeviceDisconnected(peripheral) => {
                        let mut p = self_clone.peripheral.lock().await;
                        if let Some(peri) = p.as_ref() {
                            if peri.id() == peripheral {
                                app_handle
                                    .emit("device-disconnected", peripheral.to_string())
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
        peripheral_id: peripheral.as_ref().unwrap().id().to_string(),
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
            .unwrap_or(0),
    };
    Ok(device)
}

#[tauri::command]
async fn connect(
    peripheral_id: String,
    connection: State<'_, BleConnection>,
    app_handle: AppHandle,
) -> Result<bool, String> {
    if let Err(e) = connection.connect(peripheral_id, &app_handle).await {
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

#[tauri::command]
fn get_widget_url() -> Result<serde_json::Value, String> {
    let widget_builtin_url = "http://127.0.0.1:9918/widget/builtin";
    let widget_user_url = "http://127.0.0.1:9918/widget/user";
    let widget_url = serde_json::json!({
        "builtin": widget_builtin_url,
        "user": widget_user_url
    });
    Ok(widget_url)
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

    let mut builder = tauri::Builder::default();

    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, _, _| {
            if let Some(window) = app.get_window("main") {
                if !window.is_visible().unwrap_or(true) {
                    let _ = window.show();
                }
                if window.is_minimized().unwrap_or(false) {
                    let _ = window.unminimize();
                }
                let _ = window.set_focus();
            }
        }));
    }

    builder
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .manage(BleConnection::new(central))
        .invoke_handler(tauri::generate_handler![
            register_central_events,
            start_scan,
            stop_scan,
            is_connected,
            connect,
            disconnect,
            get_connected_device,
            get_widget_url
        ])
        .setup(move |app: &mut tauri::App| {
            let app_handle: tauri::AppHandle = app.handle().clone();
            std::thread::spawn(move || {
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async move {
                    let widget_builtin_path = app_handle
                        .path()
                        .resolve("plugins", BaseDirectory::Resource)
                        .unwrap();
                    let widget_user_path = app_handle
                        .path()
                        .resolve("plugins", BaseDirectory::AppData)
                        .unwrap();

                    // /widget/builtin/{any…} → plugins/{any…}
                    let widget_builtin_route = warp::path("widget")
                        .and(warp::path("builtin"))
                        .and(warp::fs::dir(widget_builtin_path.clone()));
                    // /widget/user/{any…} → plugins/{any…}
                    let widget_user_route = warp::path("widget")
                        .and(warp::path("user"))
                        .and(warp::fs::dir(widget_user_path.clone()));

                    let routes = widget_builtin_route.or(widget_user_route);

                    warp::serve(routes).run(([127, 0, 0, 1], 9918)).await;
                });
            });

            let win_builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
                .title("Heartbeat Cat")
                .inner_size(800.0, 500.0)
                .min_inner_size(800.0, 500.0)
                .user_agent("Heartbeat Cat Desktop");

            #[cfg(target_os = "macos")]
            use Tauri::TitleBarStyle;

            #[cfg(target_os = "macos")]
            let win_builder = win_builder.title_bar_style(TitleBarStyle::Transparent);

            let _ = win_builder.build().unwrap();

            // #[cfg(target_os = "macos")]
            // {
            //     use cocoa::appkit::{NSColor, NSWindow};
            //     use cocoa::base::{id, nil};

            //     let ns_window = window.ns_window().unwrap() as id;
            //     unsafe {
            //         let bg_color = NSColor::colorWithRed_green_blue_alpha_(
            //             nil,
            //             50.0 / 255.0,
            //             158.0 / 255.0,
            //             163.5 / 255.0,
            //             1.0,
            //         );
            //         ns_window.setBackgroundColor_(bg_color);
            //     }
            // }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
