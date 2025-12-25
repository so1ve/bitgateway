import { attachConsole } from "@tauri-apps/plugin-log";
import { createApp } from "vue";

import App from "./App.vue";
import { initStore } from "./api/store";
import { initManager } from "./logic/init";
import { router } from "./router";

import "./style.css";

await initManager.start();
await initStore();

const _detach = await attachConsole();

createApp(App).use(router).mount("#app");
