<script setup>
import { computed } from '@vue/reactivity';
import { ipcRenderer } from 'electron';
import store from '../store/index';

const isConnected = computed(() => store.connection.isConnected);
const isScanning = computed(() => store.scan.isScanning);
const devicesList = computed(() => store.scan.scannedDevices);
const isPerformingConnect = computed(() => store.connection.isPerformingConnect);

const performConnect = (device) => {
    store.updateIsPerformingConnect(true);
    ipcRenderer.send('perform-connect', device);
}
</script>

<template>
    <div class="container">
        <div class="header">
            <h2 class="title">扫描和连接</h2>
            <div class="actions">
                <span class="additional" v-if="!isScanning && devicesList.length > 0" style="margin-right: 4px;">
                    {{ isConnected ? '' : `发现 ${devicesList.length} 个设备` }}
                </span>
                <button @click="$emit('scanDevice')" :disabled="isScanning || isConnected" class="btn">
                    {{ isScanning ? '正在扫描...' : '扫描'}}
                </button>
            </div>
        </div>
        <div class="content">
            <div v-if="isConnected" class="empty">
                <span>{{ `已经连接到 ${store.connection.deviceName}` }}</span>
                <div class="horizon">
                    <router-link class="btn outline" to="/hr-chart">查看心率曲线图</router-link>
                    <button class="btn outline" @click="$emit('performDisconnect')">断开连接</button>
                </div>
            </div>
            <div v-else-if="devicesList.length === 0" class="empty">
                {{isScanning ? '正在扫描支持的设备...' : '没有发现设备'}}
            </div>
            <div v-else class="device-item" v-for="device in devicesList">
                <div class="info">
                    <span class="name">{{device.deviceName}}</span>
                    <span class="id">{{device.deviceId}}</span>
                </div>
                <div class="actions">
                    <button class="btn outline" @click="performConnect({...device})"
                        :disabled="isPerformingConnect || isScanning || isConnected">连接</button>
                </div>
            </div>
        </div>
    </div>
</template>

<style>
.device-item {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
    /* padding: 8px 16px;
    border-bottom: 1px solid rgb(200, 200, 200); */
    padding: 16px 14px;
    border-radius: 4px;
    border: 1px solid rgb(200, 200, 200);
    box-shadow: 2px 2px 6px -2px rgb(200, 200, 200);
    transition: all .3s;
    margin-bottom: 10px;
}

.device-item:hover {
    box-shadow: 2px 2px 12px -2px rgb(200, 200, 200);
}

.device-item .info {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    justify-content: center;
}

.device-item .info .name {
    font-size: 16px;
    font-weight: medium;
    color: rgb(100, 100, 100);
}

.device-item .info .id {
    font-size: 12px;
    color: rgb(150, 150, 150);
}
</style>
