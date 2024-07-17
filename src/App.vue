<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { Device, useBrcatStore } from "./stores";

const store = useBrcatStore();

invoke("register_central_events");

listen("scan-list-update", (event) => {
  console.log('scan-list-update', event.payload);
})

listen("device-discovered", (event) => {
  const device = event.payload as Device
  store.pushDevice(device);
})

listen("device-connected", (event) => {
  console.log('device-connected', event.payload);
})

listen("device-disconnected", (event) => {
  console.log('device-disconnected', event.payload);
})
</script>

<template>
  <div>
    <DrawerContainer>
      <RouterView v-slot="{ Component }">
        <Transition name="scale" mode="out-in">
          <KeepAlive :exclude="['settings']">
            <component :is="Component" />
          </KeepAlive>
        </Transition>
      </RouterView>
    </DrawerContainer>
  </div>
</template>

<style>
@import './assets/css/font.css';
@import './assets/css/style.css';

:root {
  --primary-color: #F25E86;
  --app-background: #f8f8f8;

  --title-bar-background: #e4e4e4;
  --title-bar-color: var(--primary-color);
  --title-bar-height: 25px;

  --drawer-bar-background: #eeeeee;
  --drawer-bar-item-background: #eeeeee;
  --drawer-bar-item-active-background: var(--content-background);

  --content-background: var(--app-background);
}

::-webkit-scrollbar {
  --bar-width: 8px;
  width: var(--bar-width);
  height: var(--bar-width);
}

::-webkit-scrollbar-track {
  background-color: transparent;
}

::-webkit-scrollbar-thumb {
  --bar-color: rgba(0, 0, 0, .2);
  background-color: var(--bar-color);
  border-radius: 20px;
  background-clip: content-box;
  border: 1px solid transparent;
}

/* Scale */
.scale-enter-active,
.scale-leave-active {
  transition: all 200ms ease;
  -webkit-transition: all 200ms ease;
  -moz-transition: all 200ms ease;
  -ms-transition: all 200ms ease;
  -o-transition: all 200ms ease;
}


.scale-enter-from,
.scale-leave-to {
  opacity: 0;
  transform: scale(0.99);
  -webkit-transform: scale(0.99);
  -moz-transform: scale(0.99);
  -ms-transform: scale(0.99);
  -o-transform: scale(0.99);
}

html,
body {
  padding: 0;
  margin: 0;
}

body {
  font-family: Rubik, 'Noto Sans SC', sans-serif;
  font-weight: 400;
  background-color: var(--app-background);
  border-radius: 8px;
  overflow: hidden;
}

.titlebar {
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

.titlebar .icon {
  height: calc(var(--title-bar-height) - 6px);
}
</style>
