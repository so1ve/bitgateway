import { load } from "@tauri-apps/plugin-store";

import type { Credentials } from "../types";

const store = await load("credentials");

export async function initStore() {
	async function hasOrSetDefault<T>(key: string, defaultValue: T) {
		if (!(await store.has(key))) {
			await store.set(key, defaultValue);
		}
	}

	await hasOrSetDefault("username", "");
	await hasOrSetDefault("password", "");
	await hasOrSetDefault("rememberMe", false);
	await hasOrSetDefault("autoLogin", false);
	await hasOrSetDefault("autoReconnect", false);

	await store.save();
}

export const getCredentials = async () =>
	({
		username: (await store.get("username"))!,
		password: (await store.get("password"))!,
		rememberMe: (await store.get("rememberMe"))!,
		autoLogin: (await store.get("autoLogin"))!,
		autoReconnect: (await store.get("autoReconnect"))!,
	}) as Credentials;

export async function setCredentials(credentials: Credentials) {
	await store.set("username", credentials.username);
	await store.set("password", credentials.password);
	await store.set("rememberMe", credentials.rememberMe);
	await store.set("autoLogin", credentials.autoLogin);
	await store.set("autoReconnect", credentials.autoReconnect);

	await store.save();
}
