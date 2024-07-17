import { defineStore } from "pinia";
import { onMounted, ref, watchEffect } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { useThrottleFn, useDebounceFn } from "@vueuse/core";

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

  const current_heart_rate = ref<Number>(0);

  const resetCurrentHeartRate = useDebounceFn(() => {
    current_heart_rate.value = 0;
  }, 6000);

  onMounted(async () => {
    const unlisten = await listen("heart-rate", (heart_rate) => {
      resetCurrentHeartRate();
      current_heart_rate.value = heart_rate.payload as Number;
    });

    return unlisten;
  });

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

  const throttledSort = useThrottleFn(() => {
    scanning_devices.value = scanning_devices.value.sort(
      (a, b) => Math.round(b.rssi / 10) - Math.round(a.rssi / 10)
    );
  }, 2000);

  function pushDevice(device: Device) {
    if (scanning_devices.value.some((d) => d.address === device.address)) {
      scanning_devices.value = scanning_devices.value.map((d) =>
        d.address === device.address ? device : d
      );
    } else {
      scanning_devices.value.push(device);
    }

    throttledSort();
  }

  function startScan() {
    invoke("start_scan");
    is_scanning.value = true;
  }

  function stopScan() {
    invoke("stop_scan");
    is_scanning.value = false;
  }

  return {
    current_heart_rate,
    is_connected,
    is_scanning,
    connected_device,
    scanning_devices,
    pushDevice,
    startScan,
    stopScan,
  };
});
