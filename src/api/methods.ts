import { invoke } from "@tauri-apps/api/core";
import { useThrottleFn } from "@vueuse/core";

import { DEFAULT_INTERVAL } from "../constants";
import type { ApiResponse, Credentials, SrunLoginState } from "../types";

export const checkStatus = useThrottleFn(
	async () => await invoke<ApiResponse<SrunLoginState, string>>("check_status"),
	DEFAULT_INTERVAL,
);

export async function isLoggedIn() {
	const response = await checkStatus();

	return response.success && response.data.error === "ok";
}

export const setLoggedIn = useThrottleFn(
	async (loggedIn: boolean) => await invoke("set_logged_in", { loggedIn }),
	DEFAULT_INTERVAL,
);

export const login = async (credentials: Credentials) =>
	await invoke<ApiResponse<string, string>>("login", credentials as any);

export const logout = async (username: string) =>
	await invoke<ApiResponse<string, string>>("logout", {
		username,
	});
