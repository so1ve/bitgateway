import { useIntervalFn } from "@vueuse/core";
import humanize from "humanize-duration";

import { checkStatus, isLoggedIn, setLoggedIn, login } from "./api";
import { state } from "./state";
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
	let reconnecting = false;

	async function trigger() {
		const response = await checkStatus();
		const loggedIn = await isLoggedIn();

		// 自动重连逻辑
		if (
			!loggedIn &&
			state.credentials.autoReconnect &&
			state.credentials.rememberMe &&
			!state.manualLogout &&
			state.credentials.username &&
			state.credentials.password &&
			!reconnecting
		) {
			reconnecting = true;
			try {
				// 尝试重新登录
				await login(state.credentials);
				// 登录后再次检查状态
				const newResponse = await checkStatus();
				const newLoggedIn = await isLoggedIn();
				if (newLoggedIn) {
					await setLoggedIn(newLoggedIn);
					prev = newLoggedIn;
					firstCall = false;
					await cb(newLoggedIn, newResponse);
				}
			} catch (error) {
				// 重连失败，静默处理
				console.error('自动重连失败:', error);
			} finally {
				reconnecting = false;
			}
		}

		if (prev !== loggedIn || firstCall) {
			prev = loggedIn;
			firstCall = false;
			await setLoggedIn(loggedIn);
			await cb(loggedIn, response);
		}
	}

	useIntervalFn(trigger, 3000, {
		immediate: true,
		immediateCallback: true,
	});

	return trigger;
}
