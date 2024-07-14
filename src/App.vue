<script setup lang="ts">
import { listen } from "@tauri-apps/api/event";
import { RouterView } from "vue-router";
import DrawerContainer from "./components/DrawerContainer.vue";

listen("heart-rate", (hr) => {
  console.log('Heart Rate', hr);
})
</script>

<template>
  <div>
    <DrawerContainer>
      <RouterView v-slot="{ Component }">
        <Transition name="scale" mode="out-in">
          <KeepAlive>
            <component :is="Component" />
          </KeepAlive>
        </Transition>
      </RouterView>
    </DrawerContainer>
  </div>
</template>

<style>
:root {
  --app-background: #f8f8f8;

  --title-bar-background: #e4e4e4;
  --title-bar-color: #F25E86;
  --title-bar-height: 25px;

  --drawer-bar-background: #eeeeee;
  --drawer-bar-item-background: #eeeeee;
  --drawer-bar-item-active-background: var(--content-background);

  --content-background: var(--app-background);
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
  background-color: var(--app-background);
  font-family: MiSans, sans-serif;
  border-radius: 8px;
  overflow: hidden;
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

.app-top-bar .icon {
  height: calc(var(--title-bar-height) - 6px);
}
</style>
