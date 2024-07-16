<script lang="ts" setup>
import { invoke } from '@tauri-apps/api/tauri';
import { useBrcatStore } from '../stores';
import { onMounted, ref } from 'vue';

const store = useBrcatStore();
const is_connecting = ref(false);

async function connect(address: String) {
  is_connecting.value = true;
  invoke('connect', { address })
    .catch(err => {
      console.error(err);
    }).finally(() => {
      is_connecting.value = false;
    });
}

onMounted(() => {
  store.startScan();
})
</script>

<template>
  <PageContainer title="设备连接" content-class="">
    <template #actions>
      <div class="flex">
        <button class="text-[var(--primary-color)]"
          @click="() => store.is_scanning ? store.stopScan() : store.startScan()">
          <TablerReload :class="{ 'animate-spin': store.is_scanning }" />
        </button>
      </div>
    </template>
    <div v-if="store.is_connected" class="w-full h-full flex flex-col gap-4 justify-center items-center bg-white">
      <TablerBluetoothConnected class="icon text-5xl text-emerald-500" />
      <span class="flex flex-col items-center">
        <span class="text-base font-semibold">{{ store.connected_device?.name }}</span>
        <span class="text-sm font-semibold text-neutral-400">已连接到设备</span>
      </span>
      <div class="flex items-center gap-2">
        <button class="btn outline" @click="invoke('disconnect')">断开连接</button>
      </div>
    </div>
    <div class="p-4" v-else>
      <ul class="flex flex-col gap-2">
        <li class="item w-full px-3 py-3 flex justify-between items-center bg-white rounded"
          v-for="device in store.scanning_devices.filter(d => d.name !== 'Unknown')" :key="device.address">
          <div class="h-full">
            <div class="flex flex-col gap-1">
              <span class="flex items-center gap-1 text-sm leading-none">
                {{ device.name }}
              </span>
              <span class="flex items-center gap-1 text-2xs text-neutral-400">
                <SignalIndicator :rssi="device.rssi" />
                {{ device.address }}
              </span>
            </div>
          </div>
          <div class="h-full flex items-center">
            <button class="btn outline" :disabled="store.is_connected" @click="connect(device.address)">连接</button>
          </div>
        </li>
      </ul>
    </div>
  </PageContainer>
</template>

<style scoped>
.item {
  border: 1px solid rgb(220, 220, 220);
  box-shadow: 2px 2px 6px -2px rgba(200, 200, 200, 0.385);
}
</style>
