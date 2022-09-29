<script setup>
import { ipcRenderer } from 'electron';
import { join } from 'path';
import process from 'process';
import { ref, toRaw } from 'vue';

const widgetMetas = ref([]);

ipcRenderer.invoke('request-widgets').then(result => {
    widgetMetas.value = result;
});
</script>

<template>
    <div class="container">
        <div class="header">
            <h2 class="title">桌面组件</h2>
            <div class="actions">
                <button class="btn outline" @click="ipcRenderer.send('create-widget')">
                    关闭组件
                </button>
                <!-- TODO: 打开组件文件夹 -->
                <button class="btn outline" disabled
                    @click="ipcRenderer.send('open-explorer', join(process.env.DIST, 'addons/widgets'))">
                    打开组件文件夹
                </button>
                <button class="btn" disabled>
                    导入
                </button>
            </div>
        </div>
        <div class="content">
            <div class="device-item" v-for="widget in widgetMetas">
                <div class="info">
                    <span class="name">{{widget.title}}</span>
                    <span class="id">{{widget.description}}</span>
                </div>
                <div class="actions">
                    <button class="btn outline" @click="ipcRenderer.send('create-widget', toRaw(widget))">使用</button>
                </div>
            </div>
        </div>
    </div>
</template>

<style>

</style>
