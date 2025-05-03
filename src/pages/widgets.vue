<script lang="ts" setup>
import { resolveResource } from '@tauri-apps/api/path'
import { Window } from '@tauri-apps/api/window'
import { Webview } from '@tauri-apps/api/webview'
import { useSnackbar } from 'vue3-snackbar'
import { invoke } from '@tauri-apps/api/core'

const snackbar = useSnackbar()

const createWindow = async () => {
  // const url = await resolveResource('addons/widgets/example/widget.html')
  const url: string = await invoke('get_widget_url')
  console.log(url)

  const window = new Window('widget_example', {
    width: 100,
    height: 50,
    resizable: false,
    decorations: false,
    fullscreen: false,
    alwaysOnTop: true,
    title: '桌面组件',
  })

  window.once('tauri://created', () => {
    const webview = new Webview(window, 'widget_example', {
      url,
      x: 0,
      y: 0,
      width: 300,
      height: 300,
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
    <div class="w-full h-full flex justify-center items-center bg-white">
      <button
        class="btn"
        @click="createWindow"
      >
        Create Window
      </button>
    </div>
  </PageContainer>
</template>

<style scoped></style>
