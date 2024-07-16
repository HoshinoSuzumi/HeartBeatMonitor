<script lang="ts" setup>
import { invoke } from '@tauri-apps/api/tauri';
import { message } from '@tauri-apps/api/dialog';
import { useBrcatStore } from '../stores';
import { computed, onMounted, ref } from 'vue';

const store = useBrcatStore();
const is_connecting = ref(false);

const scanning_devices = computed(() => store.scanning_devices.filter(d => d.name !== 'Unknown'));

async function connect(address: String) {
  is_connecting.value = true;
  store.stopScan();
  invoke('connect', { address })
    .catch(err => {
      message(`${err}`, {
        type: 'error',
        title: '连接设备失败'
      });
      store.startScan();
    }).finally(() => {
      setTimeout(() => {
        is_connecting.value = false;
      }, 500);
    });
}

onMounted(() => {
  store.startScan();
})
</script>

<template>
  <PageContainer title="设备连接" content-class="relative">
    <template #actions>
      <div class="flex">
        <button class="text-[var(--primary-color)]"
          @click="() => store.is_scanning ? store.stopScan() : store.startScan()">
          <TablerReload :class="{ 'animate-spin': store.is_scanning }" />
        </button>
      </div>
    </template>
    <Transition name="fade">
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
      <div v-else-if="store.is_scanning && scanning_devices.length === 0"
        class="w-full h-full flex flex-col gap-4 justify-center items-center">
        <SvgSpinnersPulse2 class="icon text-5xl text-neutral-400" />
        <span class="text-sm font-semibold text-neutral-400">正在扫描蓝牙设备</span>
      </div>
      <div v-else-if="is_connecting"
        class="w-full h-full flex flex-col gap-4 justify-center items-center">
        <SvgSpinnersWifiFade class="icon text-5xl text-neutral-400" />
        <span class="text-sm font-semibold text-neutral-400">正在连接到设备</span>
      </div>
      <div class="p-4" v-else>
        <div class="flex flex-col gap-2 relative">
          <TransitionGroup name="scan-device">
            <ScanningDevice v-for="(device, _) in scanning_devices.filter(d => d.name !== 'Unknown')"
              :key="device.address" :device="device" @connect="connect" />
          </TransitionGroup>
        </div>
      </div>
    </Transition>
  </PageContainer>
</template>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

.fade-leave-active {
  transition: transform 0.3s;
  position: absolute;
  inset: 0;
}

.scan-device-enter-active,
.scan-device-leave-active {
  transition: opacity 0.3s;
}

.scan-device-enter-from,
.scan-device-leave-to {
  opacity: 0;
}

.scan-device-move {
  transition: transform 0.3s;
}
</style>
