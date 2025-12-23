import { useIntervalFn } from "@vueuse/core";
import humanize from "humanize-duration";

import { checkStatus, isLoggedIn, setLoggedIn } from "./api";
import type { ApiResponse, SrunLoginState } from "./types";

export const humanizeDuration = (ms: number) =>
	humanize(ms, {
		language: "zh_CN",
		units: ["d", "h", "m", "s"],
		delimiter: " ",
	});

function formatNumber(num: number, count: number) {
	const n = 10 ** count;
	const t = Math.floor(num * n);

	return t / n;
}

// 从深澜抄过来的，，
export function formatFlux(byte: number) {
	if (byte > 1024 * 1024 * 1024 * 1024) {
		return `${formatNumber(byte / (1024 * 1024 * 1024 * 1024), 2)}TB`;
	}
	if (byte > 1024 * 1024 * 1024) {
		return `${formatNumber(byte / (1024 * 1024 * 1024), 2)}GB`;
	}
	if (byte > 1024 * 1024) {
		return `${formatNumber(byte / (1024 * 1024), 2)}MB`;
	}
	if (byte > 1024) {
		return `${formatNumber(byte / 1024, 2)}KB`;
	}

	return `${byte}B`;
}

export async function useCheckStatus(
	cb: (
		loggedIn: boolean,
		status: ApiResponse<SrunLoginState, string>,
	) => Promise<void>,
) {
	let firstCall = true;
	let prev = await isLoggedIn();

	async function trigger() {
		const response = await checkStatus();
		const loggedIn = await isLoggedIn();

		if (prev !== loggedIn || firstCall) {
			await setLoggedIn(loggedIn);
			await cb(loggedIn, response);
			prev = loggedIn;
			firstCall = false;
		}
	}

	useIntervalFn(trigger, 3000, {
		immediate: true,
		immediateCallback: true,
	});

	return trigger;
}
