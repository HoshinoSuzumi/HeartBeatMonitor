<script lang="ts" setup>
import { resolveResource } from '@tauri-apps/api/path'
import { Effect, Window } from '@tauri-apps/api/window'
import { Webview } from '@tauri-apps/api/webview'
import { useSnackbar } from 'vue3-snackbar'
import { invoke } from '@tauri-apps/api/core'
import { usePluginManager } from '../stores/plugin'

const pluginMgr = usePluginManager()
const snackbar = useSnackbar()

const createWindow = async () => {
  // const url = await resolveResource('addons/widgets/example/widget.html')
  const url: string = await invoke('get_widget_url')
  console.log(url)

  const window = new Window('widget_example', {
    width: 100,
    height: 100,
    resizable: false,
    decorations: false,
    transparent: true,
    shadow: false,
    alwaysOnTop: true,
    title: '桌面组件',
  })

  window.once('tauri://created', () => {
    const webview = new Webview(window, 'widget_example', {
      url,
      x: 0,
      y: 0,
      width: 100,
      height: 100,
      transparent: true,
      acceptFirstMouse: true,
    })
    webview.once('tauri://error', (e) => {
      snackbar.add({
        type: 'error',
        text: `创建窗口失败: ${e.payload}`,
      })
    })
  })
}
</script>

<template>
  <PageContainer title="桌面组件">
    <div class="w-full h-full flex flex-col bg-white">
      <!-- <button
        class="btn"
        @click="createWindow"
      >
        Create Window
      </button> -->
      <button
        class="btn"
        @click="pluginMgr.refreshPlugins"
      >
        Refresh Plugins
      </button>
      <pre>{{ pluginMgr.plugins }}</pre>
    </div>
  </PageContainer>
</template>

<style scoped></style>
