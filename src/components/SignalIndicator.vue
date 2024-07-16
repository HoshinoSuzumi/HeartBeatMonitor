<script lang="ts" setup>
import { computed } from 'vue';

const props = defineProps({
  rssi: {
    type: Number,
    required: true
  }
})

const strength = computed(() => {
  if (props.rssi >= -50) return 5;
  else if (props.rssi >= -60) return 4;
  else if (props.rssi >= -70) return 3;
  else if (props.rssi >= -80) return 2;
  else return 1;
})
</script>

<template>
  <div class="flex items-center gap-[3px]">
    <span v-for="i in 5" :key="i" class="w-1 h-2 rounded-full"
      :class="{ 'bg-emerald-500': i <= strength, 'bg-neutral-200': i > strength }"></span>
  </div>
</template>

<style scoped></style>
