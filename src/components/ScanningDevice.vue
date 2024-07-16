<script lang="ts" setup>
import { PropType } from 'vue';
import { Device, useBrcatStore } from '../stores';

defineProps({
  device: {
    type: Object as PropType<Device>,
    required: true
  }
});
const emit = defineEmits(['connect']);
const store = useBrcatStore();
</script>

<template>
  <div class="item w-full px-3 py-3 flex justify-between items-center bg-white rounded" :key="device.address">
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
      <button class="btn outline" :disabled="store.is_connected" @click="emit('connect', device.address)">连接</button>
    </div>
  </div>
</template>

<style scoped>
.item {
  border: 1px solid rgb(220, 220, 220);
  box-shadow: 2px 2px 6px -2px rgba(200, 200, 200, 0.385);
}
</style>
