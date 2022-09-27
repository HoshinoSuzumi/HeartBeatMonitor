<script setup>
import { ref, watch, computed } from 'vue';
import { useRoute } from 'vue-router';
import { ipcRenderer } from 'electron';
import store from './store/index';

const route = useRoute()

const refDeviceNameWrapper = ref()
const refDeviceName = ref()

const navList = ref([
  { title: '设备连接', path: '/' },
  { title: '心率曲线', path: '/hr-chart' },
  { title: '心率组件', path: '/plugin' },
  { title: '设置', path: '/settings' },
])

const isConnected = computed(() => store.connection.isConnected);
const connectedDeviceName = computed(() => store.connection.deviceName);
const isPerformingConnect = computed(() => store.connection.isPerformingConnect);

const ble = ref(null)
const bleCharacteristic = ref(null)
const nextScan = ref(true)

ipcRenderer.on('ble-scan-devices', (event, devices) => {
  // store.updateScannedDevices(devices);
  store.addOrUpdateScannedDevice(devices[0]);
  nextScan.value = true;
})

ipcRenderer.on('require-connect-request', (ev, deviceInfo) => {
  ble.value = navigator.bluetooth.requestDevice({
    filters: [{ services: ['heart_rate'] }],
  })
  ble.value.then(device => {
    console.log('connect', device);
    return device.gatt.connect();
  }).then(server => {
    console.log('connect', server);
    return server.getPrimaryService('heart_rate');
  }).then(service => {
    console.log('connect', service);
    return service.getCharacteristic('heart_rate_measurement');
  }).then(character => {
    console.log('connect', character);
    bleCharacteristic.value = character;
    bleCharacteristic.value.startNotifications().then(_ => {
      store.updateIsConnected(true);
      store.updateConnectedDeviceName(deviceInfo.deviceName);
      store.updateConnectedDeviceId(deviceInfo.deviceId);
      bleCharacteristic.value.addEventListener('characteristicvaluechanged', handleCharacteristicValueChanged);
    })
  }).catch(error => {
    store.updateIsConnected(false);
    console.log('connect', error);
  }).finally(() => {
    store.updateIsPerformingConnect(false);
  })
})

const handleCharacteristicValueChanged = (ev) => {
  let hrValue = ev.target.value.getUint8(1);
  store.updateHR(hrValue);
}

const scanDevice = () => {
  console.log('performing scan');
  store.updateIsScanning(true);
  store.clearScannedDevices();
  let scanInterval = setInterval(() => {
    if (nextScan.value) {
      nextScan.value = false
      ble.value = navigator.bluetooth.requestDevice({
        filters: [{ services: ['heart_rate'] }],
        // acceptAllDevices: true,
      })
      ble.value.then(device => {
        device.gatt.connect();
        // nextScan.value = false
      }).catch(e => {
        if (e.message.indexOf('Must be handling a user gesture') !== -1) {
          console.log('Need user gueture');
          clearInterval(scanInterval)
          nextScan.value = true
          store.updateIsScanning(false)
        } else if (e.message.indexOf('cancelled' !== -1)) {
          // pass
        } else {
          console.log(e);
        }
      })
    }
  }, 1000);
}

watch(connectedDeviceName, v => {
  setTimeout(() => {
    console.log('connectedDeviceName', v);
    if (refDeviceName.value.clientWidth > refDeviceNameWrapper.value.clientWidth) {
      refDeviceName.value.classList.add('infinite-scroll')
      refDeviceName.value.style.setProperty('--scroll-width', `-${refDeviceName.value.clientWidth - refDeviceNameWrapper.value.clientWidth}px`)
    } else {
      refDeviceName.value.classList.remove('infinite-scroll')
    }
  }, 100);
})
</script>

<template>
  <div>
    <div class="app-top-bar">BLE Monitor</div>

    <div class="view-wrapper">
      <div class="drawer-bar">
        <div class="navs">
          <router-link v-for="nav, i in navList" :to="nav.path" class="nav" :class="{
          'active': route.path === nav.path, 
          'inactive-before': navList[i-1] && navList[i-1].path === route.path, 
          'inactive-after': navList[i+1] && navList[i+1].path === route.path
          }">
            {{ nav.title }}
          </router-link>
        </div>
        <div class="status" ref="refStatusBlock">
          <div class="dot-wrapper">
            <div class="dot"
              :class="{'scanning': isPerformingConnect, 'disconnected': !isConnected && !isPerformingConnect, 'connected': isConnected}">
            </div>
          </div>
          <span class="device-wrapper" ref="refDeviceNameWrapper">
            <span class="device" ref="refDeviceName">{{ isPerformingConnect ? '正在连接...' : isConnected ? connectedDeviceName : '未连接'
            }}&nbsp;&nbsp;</span>
          </span>
        </div>
      </div>
      <router-view v-slot="{Component}" @scan-device="scanDevice">
        <transition name="scale" mode="out-in">
          <keep-alive :exclude="['hr-chart']">
            <component :is="Component" />
          </keep-alive>
        </transition>
      </router-view>
    </div>
  </div>
</template>

<style>
@import './assets/css/style.css';
@import './assets/css/MiSans.css';

html,
body {
  padding: 0;
  margin: 0;
}

body {
  background-color: var(--app-background);
  font-family: MiSans, sans-serif;
}

.app-top-bar {
  -webkit-user-select: none;
  user-select: none;
  -webkit-app-region: drag;
  position: fixed;
  z-index: 99;
  width: 100%;
  height: var(--title-bar-height);
  display: flex;
  flex-direction: row;
  align-items: center;
  padding: 0 8px;
  background-color: var(--title-bar-background);
  color: var(--title-bar-color);
  font-size: 12px;
  font-weight: bold;
  overflow: hidden;
}

.view-wrapper {
  width: 100%;
  height: calc(100vh - var(--title-bar-height));
  padding-top: var(--title-bar-height);
  display: flex;
  flex-direction: row;
}

.drawer-bar {
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  position: fixed;
  top: var(--title-bar-height);
  bottom: 0;
  width: 120px;
  background-color: var(--drawer-bar-background);
}

.drawer-bar .navs {
  display: flex;
  flex-direction: column;
  background-color: var(--drawer-bar-item-active-background);
}

.drawer-bar .navs .nav {
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 14px 0;
  color: rgb(116, 125, 136);
  font-size: 14px;
  border-radius: 0;
  background-color: var(--drawer-bar-background);
}

.drawer-bar .navs .nav.inactive-before {
  border-radius: 0 10px 0 0;
  background-color: var(--drawer-bar-background);
}

.drawer-bar .navs .nav.inactive-after {
  border-radius: 0 0 10px 0;
  background-color: var(--drawer-bar-background);
}

.drawer-bar .navs .nav.active {
  border-radius: 0;
  color: rgb(0, 0, 0);
  background-color: var(--drawer-bar-item-active-background);
  font-weight: bold;
}

.status {
  display: flex;
  flex-direction: row;
  align-items: center;
  width: 100%;
  height: 30px;
  font-size: 12px;
  border-top: 1px solid #d5d5d5;
  overflow-x: hidden;
}

.status .dot-wrapper {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 30px;
  width: 30px;
}

.status .dot-wrapper .dot {
  height: 10px;
  width: 10px;
  border-radius: 50%;
  /* background-color: rgb(255, 100, 100); */
  background-color: rgb(180, 180, 180);
}

.status .dot-wrapper .dot.connected {
  background-color: rgb(100, 255, 100);
}

.status .dot-wrapper .dot.disconnected {
  background-color: rgb(255, 100, 100);
}

.status .dot-wrapper .dot.scanning {
  background-color: rgb(229, 232, 40);
}

.status .device-wrapper {
  display: flex;
  align-items: center;
  width: calc(100% - 30px);
  height: 100%;
  overflow-x: hidden;
}

.status .device-wrapper .device {
  white-space: nowrap;
  --scroll-width: 0;
}

@keyframes text-scroll {
  0% {
    transform: translateX(0);
  }

  30% {
    transform: translateX(var(--scroll-width));
  }

  50% {
    transform: translateX(var(--scroll-width));
  }

  80% {
    transform: translateX(0%);
  }

  100% {
    transform: translateX(0%);
  }

}

.infinite-scroll {
  animation: text-scroll 5s ease-in-out infinite;
}
</style>
