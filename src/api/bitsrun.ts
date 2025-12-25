import { invoke } from "@tauri-apps/api/core";

import { state } from "../state";
import type { ApiResponse } from "../types";

const INIT_RETRY_INTERVAL = 3000;
let initRetryTimeout: ReturnType<typeof setTimeout> | null = null;
let initInFlight = false;

function clearInitRetry() {
	if (initRetryTimeout !== null) {
		clearTimeout(initRetryTimeout);
		initRetryTimeout = null;
	}
}

function scheduleInitRetry() {
	if (initRetryTimeout !== null || state.initialized) {
		return;
	}

	initRetryTimeout = setTimeout(() => {
		initRetryTimeout = null;
		void initBitsrun();
	}, INIT_RETRY_INTERVAL);
}

async function attemptInit() {
	const response = await invoke<ApiResponse<string, string>>("init");
	if (response.success) {
		state.initialized = true;
		clearInitRetry();

		return true;
	}

	state.initializeMessage = response.error;

	return false;
}

export async function initBitsrun() {
	if (state.initialized || initInFlight) {
		return;
	}

	initInFlight = true;
	const success = await attemptInit();
	initInFlight = false;

	if (!success) {
		scheduleInitRetry();
	}
}
