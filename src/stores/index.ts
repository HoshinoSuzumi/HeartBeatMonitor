import { defineStore } from "pinia";
import { ref, watchEffect } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

export interface Device {
  name: string;
  address: string;
  rssi: number;
}

export const useBrcatStore = defineStore("brcat", () => {
  const scanning_devices = ref<Device[]>([]);

  const is_connected = ref(false);
  const connected_device = ref<Device | null>(null);

  const is_scanning = ref(false);

  setInterval(async () => {
    is_connected.value = await invoke("is_connected");
  }, 500);

  watchEffect(async () => {
    scanning_devices.value = [];
    if (is_connected.value) {
      connected_device.value = await invoke("get_connected_device");
    } else {
      connected_device.value = null;
      is_scanning.value = false;
      setTimeout(() => {
        startScan();
      }, 500);
    }
    stopScan();
  });

  function pushDevice(device: Device) {
    if (scanning_devices.value.some((d) => d.address === device.address)) {
      scanning_devices.value = scanning_devices.value.map((d) =>
        d.address === device.address ? device : d
      );
    } else {
      scanning_devices.value.push(device);
    }
    // scanning_devices.value = scanning_devices.value.sort(
    //   (a, b) => b.rssi - a.rssi
    // );
  }

  function startScan() {
    invoke("start_scan");
    is_scanning.value = true;
    console.log("start scan");
  }

  function stopScan() {
    invoke("stop_scan");
    is_scanning.value = false;
    console.log("stop scan");
  }

  return {
    is_connected,
    is_scanning,
    connected_device,
    scanning_devices,
    pushDevice,
    startScan,
    stopScan,
  };
});
