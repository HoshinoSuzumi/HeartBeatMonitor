// make this js file as a module
import { listen } from "@tauri-apps/api/event";

listen('heart-rate', event => {
  console.log('Heart rate:', event.payload)
})

export {}
