import Tailwindcss from "@tailwindcss/vite";
import Vue from "@vitejs/plugin-vue";
import { defineConfig } from "vite";
import Font from "vite-plugin-font";
import WebfontDownload from "vite-plugin-webfont-dl";

const host = process.env.TAURI_DEV_HOST;
// https://vite.dev/config/

export default defineConfig(async () => ({
	plugins: [
		Vue(),
		Tailwindcss(),
		WebfontDownload([
			"https://fonts.googleapis.com/css2?family=Noto+Sans+SC&display=swap",
			"https://fonts.googleapis.com/css2?family=Geist&display=swap",
			"https://fonts.googleapis.com/css2?family=Geist+Mono&display=swap",
		]),
		Font.vite(),
	],

	// Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
	//
	// 1. prevent Vite from obscuring rust errors
	clearScreen: false,
	// 2. tauri expects a fixed port, fail if that port is not available
	server: {
		port: 1420,
		strictPort: true,
		host: host ?? false,
		hmr: host
			? {
					protocol: "ws",
					host,
					port: 1421,
				}
			: undefined,
		watch: {
			// 3. tell Vite to ignore watching `src-tauri`
			ignored: ["**/src-tauri/**"],
		},
	},
}));
