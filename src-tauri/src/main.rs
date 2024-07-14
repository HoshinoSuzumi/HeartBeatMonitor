// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AppHandle, Manager, State};
use tokio;
use tokio::sync::{Mutex};
use btleplug::api::{Central, ScanFilter, Manager as _, Peripheral as _};
use btleplug::api::bleuuid::uuid_from_u16;
use btleplug::platform::{Adapter, Manager as BtleManager, Peripheral};
use futures::StreamExt;

#[derive(serde::Serialize, Clone)]
struct BleDevice {
  name: String,
  address: String,
}

struct BleConnection {
  manager: Mutex<Option<BtleManager>>,
  adapter: Mutex<Option<Adapter>>,
  peripheral: Mutex<Option<Peripheral>>,
}

#[tauri::command]
async fn scan_devices(
  connection: State<'_, BleConnection>
) -> Result<String, String> {
  let adapter = connection.adapter.lock().await;
  let adapter = adapter.as_ref().unwrap();
  adapter.start_scan(ScanFilter::default()).await.unwrap();
  tokio::time::sleep(std::time::Duration::from_secs(2)).await;

  let mut devices = vec![];
  for p in adapter.peripherals().await.unwrap() {
    devices.push(BleDevice {
      name: p.properties().await.unwrap().unwrap().local_name.unwrap_or("Unknown".to_string()),
      address: p.address().to_string(),
    });
  }

  Ok(serde_json::to_string(&devices).unwrap())
}

#[tauri::command]
async fn connect(
  address: String,
  connection: State<'_, BleConnection>,
  app_handle: AppHandle,
) -> Result<bool, String> {
  let adapter = connection.adapter.lock().await;
  let adapter = adapter.as_ref().unwrap();
  // adapter.start_scan(ScanFilter::default()).await.unwrap();
  // tokio::time::sleep(std::time::Duration::from_secs(2)).await;

  let peripheral = adapter.peripherals().await.unwrap().into_iter().find(|p| p.address().to_string() == address).unwrap().clone();
  peripheral.connect().await.unwrap();

  *connection.peripheral.lock().await = Some(peripheral.clone());

  peripheral.discover_services().await.unwrap_or_else(|_| {
    println!("Failed to discover services");
  });
  let service = peripheral.services()
    .into_iter()
    .find(|s| s.uuid == uuid_from_u16(0x180D))
    .unwrap();
  let characteristic = service.characteristics
    .into_iter()
    .find(|c| c.uuid == uuid_from_u16(0x2A37))
    .unwrap();

  peripheral.subscribe(&characteristic).await.unwrap_or_else(|_| {
    println!("Failed to subscribe to characteristic");
  });

  tokio::spawn(async move {
    let mut notification_stream = peripheral.notifications().await.unwrap();
    while let Some(notification) = notification_stream.next().await {
      if notification.uuid == uuid_from_u16(0x2A37) {
        app_handle.emit_all("heart-rate", notification.value[1]).unwrap();
      }
    }
  });

  Ok(true)
}

#[tauri::command]
async fn disconnect(
  connection: State<'_, BleConnection>
) -> Result<bool, String> {
  let connection = connection.peripheral.lock().await;
  let connection = connection.as_ref().unwrap();

  connection.disconnect().await.unwrap();

  Ok(true)
}

#[tauri::command]
async fn get_connected_device(
  connection: State<'_, BleConnection>
) -> Result<String, String> {
  let connection = connection.peripheral.lock().await;
  let connection = connection.as_ref().unwrap();

  Ok(serde_json::to_string(&BleDevice {
    name: connection.properties().await.unwrap().unwrap().local_name.unwrap_or("Unknown".to_string()),
    address: connection.address().to_string(),
  }).unwrap())
}

#[tokio::main]
async fn main() {
  let ble_manager = BtleManager::new().await.unwrap();
  let adapter = ble_manager.adapters().await.unwrap().into_iter().next().unwrap();

  tauri::Builder::default()
    .manage(BleConnection {
      manager: Mutex::new(Some(ble_manager)),
      adapter: Mutex::new(Some(adapter)),
      peripheral: Default::default(),
    })
    .invoke_handler(tauri::generate_handler![
      scan_devices, 
      connect,
      disconnect,
      get_connected_device
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
