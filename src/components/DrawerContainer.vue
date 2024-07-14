<script lang="ts" setup>
import { ref } from 'vue'

const navList = ref([
  { title: '设备连接', path: '/' },
  { title: '心率曲线', path: '/hr-chart' },
  { title: '桌面组件', path: '/widgets' },
  { title: '推流插件', path: '/streaming-plugin' },
  { title: '设置', path: '/settings' },
])
</script>

<template>
  <div class="drawer">
    <div class="side">
      <div class="nav">
        <router-link
          v-for="item in navList"
          :key="item.path"
          :to="item.path"
          class="nav-item"
          active-class="active"
        >
          {{ item.title }}
        </router-link>
      </div>
      <div class="status"></div>
    </div>
    <slot />
  </div>
</template>

<style scoped>
.drawer {
  display: flex;
  height: 100%;
}
</style>
