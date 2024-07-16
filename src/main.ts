import { createApp } from "vue";
import { createRouter, createWebHistory } from "vue-router";

import App from "./App.vue";

const routes = [
  { path: "/", component: () => import("./pages/index.vue") },
  { path: "/charts", component: () => import("./pages/charts.vue") },
  { path: "/widgets", component: () => import("./pages/widgets.vue") },
  { path: "/streaming-plugins", component: () => import("./pages/streaming-plugins.vue") },
  { path: "/settings", component: () => import("./pages/settings.vue") },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

createApp(App).use(router).mount("#app");
