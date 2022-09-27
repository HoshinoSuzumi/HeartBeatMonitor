import { createApp } from 'vue'
import { createRouter, createWebHashHistory } from 'vue-router'

import App from './App.vue'
import index from './pages/index.vue'
import hrChart from './pages/hr-chart.vue'
import plugin from './pages/plugin.vue'
import settings from './pages/settings.vue'

const routes = [
  { path: '/', component: index },
  { path: '/hr-chart', component: hrChart },
  { path: '/plugin', component: plugin },
  { path: '/settings', component: settings },
]

const router = createRouter({
  history: createWebHashHistory(),
  routes
})

createApp(App)
  .use(router)
  .mount('#app')
  .$nextTick(() => {
    postMessage({ payload: 'removeLoading' }, '*')
  })
