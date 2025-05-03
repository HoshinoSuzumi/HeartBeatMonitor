<script setup lang="ts">
import { listen } from '@tauri-apps/api/event'
import { ref } from 'vue'

const heartRate = ref(0)

listen('heart-rate', (event) => {
  console.log('Heart rate event received:', event.payload)
  heartRate.value = event.payload as number
})
</script>

<template>
  <div
    data-tauri-drag-region
    class="widget"
  >
    <span>{{ heartRate }}</span>
  </div>
</template>

<style scoped>
.widget {
  width: 100px;
  height: 50px;
  display: flex;
  justify-content: center;
  align-items: center;
  border-radius: 8px;
  background-color: #ffa1c6;
  user-select: none;
  overflow: hidden;
}
</style>
