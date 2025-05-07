<script setup lang="ts">
const isDev = import.meta.env.DEV
</script>

<template>
  <slot v-if="isDev" />
</template>
