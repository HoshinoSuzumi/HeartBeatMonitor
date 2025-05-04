<script lang="ts" setup>
import { PropType } from 'vue'
import { PluginManifest } from '../stores/plugin'

defineProps({
  plugin: {
    type: Object as PropType<PluginManifest>,
    required: true,
  },
})
const emit = defineEmits<{
  (e: 'activate-widget' | 'close-widget', plugin: PluginManifest): void
}>()
</script>

<template>
  <div
    class="item w-full px-3 py-3 flex justify-between items-center bg-white rounded"
    :key="plugin.name"
  >
    <div class="h-full">
      <div class="flex flex-col gap-1.5">
        <span class="flex items-center gap-1 text-sm leading-none">
          <span
            v-if="plugin.isBuiltIn"
            class="badge"
          >
            内置
          </span>
          {{ plugin.name }}
        </span>
        <span class="flex items-center gap-2 text-2xs text-neutral-400">
          <span> V{{ plugin.version }} </span>
          <span>
            {{ plugin.description || '没有描述' }}
          </span>
        </span>
      </div>
    </div>
    <div class="h-full flex items-center">
      <button
        class="btn outline"
        @click="
          emit(plugin.isActivated ? 'close-widget' : 'activate-widget', plugin)
        "
      >
        {{ plugin.isActivated ? '关闭' : '启用' }}
      </button>
    </div>
  </div>
</template>

<style scoped>
.item {
  border: 1px solid rgb(220, 220, 220);
  box-shadow: 2px 2px 6px -2px rgba(200, 200, 200, 0.385);
}

.badge {
  @apply text-2xs font-semibold text-neutral-500;
  @apply bg-neutral-100 rounded-sm p-0.5;
  @apply border border-neutral-200;
}
</style>
