import { invoke } from "@tauri-apps/api/core";
import { useThrottleFn } from "@vueuse/core";

import type { ApiResponse, Credentials, SrunLoginState } from "../types";

export const checkStatus = useThrottleFn(
	async () => await invoke<ApiResponse<SrunLoginState, string>>("check_status"),
	1000,
);

export async function isLoggedIn() {
	const response = await checkStatus();

	return response.success && response.data.error === "ok";
}

export const setLoggedIn = useThrottleFn(
	async (loggedIn: boolean) => await invoke("set_logged_in", { loggedIn }),
	1000,
);

export const login = async (credentials: Credentials) =>
	await invoke<ApiResponse<string, string>>("login", credentials as any);

export const logout = async (username: string) =>
	await invoke<ApiResponse<string, string>>("logout", {
		username,
	});

export { initBitsrun } from "./init";
export { getCredentials, initStore, setCredentials } from "./store";
