<script setup lang="ts">
import { ref, onBeforeUnmount } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const address = ref("");
const device_info = ref('');

async function connect() {
  invoke("connect", { address: address.value }).then(res => {
    console.log(res);
  }).catch(err => {
    console.error(err);
  });
}

async function disconnect() {
  invoke("disconnect").then(res => {
    console.log(res);
  }).catch(err => {
    console.error(err);
  });
}

async function scan() {
  invoke("scan_devices").then(res => {
    console.log(JSON.parse(res as string));
  })
}

async function device() {
  invoke("get_connected_device").then(res => {
    device_info.value = res as string;
  })
}
</script>

<template>
  <form class="row" @submit.prevent="connect">
    <input v-model="address" placeholder="Enter a address..." />
    <button type="submit">Connect</button>
    <button @click="disconnect" type="button">Discon</button>
    <button @click="scan" type="button">Scan</button>
    <button @click="device" type="button">Device</button>
  </form>

  <pre>{{ device_info }}</pre>
</template>
