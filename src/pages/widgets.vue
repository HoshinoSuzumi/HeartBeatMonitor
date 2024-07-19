<script lang="ts" setup>
import { resolveResource } from '@tauri-apps/api/path';
import { WebviewWindow } from '@tauri-apps/api/window';
import { useSnackbar } from 'vue3-snackbar';

const snackbar = useSnackbar();

const createWindow = async () => {
  const url = (await resolveResource('addons/widgets/example/widget.html')).replace('\\\\?\\', 'file://')
  console.log(url);
  const webview = new WebviewWindow('widget_example', {
    url,
    width: 400,
    height: 400
  })
  webview.once('tauri://error', e => {
    snackbar.add({
      type: 'error',
      text: `创建窗口失败: ${e.payload}`
    })
  })
}
</script>

<template>
  <PageContainer title="桌面组件">
    <div class="w-full h-full flex justify-center items-center bg-white">
      <button class="btn" @click="createWindow">Create Window</button>
    </div>
  </PageContainer>
</template>

<style scoped></style>
