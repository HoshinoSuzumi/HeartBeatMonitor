<script lang="ts" setup>
defineProps({
  title: String,
  subtitle: {
    type: String,
    default: null
  },
  contentClass: {
    type: String,
    default: ''
  }
})
</script>

<template>
  <div class="flex flex-col h-full">
    <header
      class="w-full h-11 px-4 flex justify-between items-center bg-gradient-to-r from-pink-50 to-pink-100 border-b">
      <div class="flex flex-col gap-0.5">
        <h1 class="font-semibold text-neutral-900" :class="{ 'text-xs': !!subtitle, 'text-base': !subtitle }">
          {{ title }}
        </h1>
        <p v-if="subtitle" class="text-2xs text-neutral-500">
          {{ subtitle }}
        </p>
      </div>
      <div class="flex items-center">
        <slot name="actions" />
      </div>
    </header>
    <div :class="`flex-1 overflow-auto relative ${contentClass}`">
      <slot />
    </div>
  </div>
</template>

<style scoped></style>
