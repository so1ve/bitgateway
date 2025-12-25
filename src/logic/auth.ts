import { toast } from "vue-sonner";

import { login as apiLogin, logout as apiLogout } from "../api/methods";
import { setCredentials } from "../api/store";
import { router } from "../router";
import { state } from "../state";
import type { SrunLoginStateLoggedIn } from "../types";
import { monitorStatus } from "./status";

class AuthManager {
	private isRunning = false;

	public async start() {
		if (this.isRunning) {
			return;
		}
		this.isRunning = true;

		for await (const status of monitorStatus()) {
			switch (status.type) {
				case "online": {
					state.loggedIn = true;
					state.userInfo = status.data;
					state.statusMessage = "已连接";

					if (router.currentRoute.value.path === "/") {
						router.push("/status");
					}
					break;
				}

				case "offline": {
					state.loggedIn = false;
					state.userInfo = null;
					state.statusMessage = "未登录";

					if (router.currentRoute.value.path === "/status") {
						router.push("/");
					}

					if (
						state.credentials.autoLogin &&
						!state.manualLogout &&
						!state.loggingIn
					) {
						await this.login(true);
					}
					break;
				}

				case "error": {
					state.statusMessage = `连接错误: ${status.error}`;
					// In case of error, we don't force logout in UI immediately to avoid flickering,
					// but we also don't try to login because network is likely down.
					break;
				}
			}
		}
	}

	public async login(isAuto = false) {
		if (state.loggingIn) {
			return;
		}
		state.loggingIn = true;
		state.statusMessage = isAuto ? "自动登录中..." : "登录中...";

		try {
			if (state.credentials.rememberMe) {
				await setCredentials(state.credentials);
			}

			const response = await apiLogin(state.credentials);
			if (response.success) {
				state.manualLogout = false;
				state.loggedIn = true;
				state.firstOpen = false;
				if (!isAuto) {
					toast.success("登录成功");
				}
				// We can redirect immediately for better responsiveness
				if (router.currentRoute.value.path === "/") {
					router.push("/status");
				}
			} else {
				state.statusMessage = `登录失败: ${response.error}`;
				if (isAuto) {
					console.error("Auto login failed:", response.error);
				} else {
					toast.error(`登录失败: ${response.error}`);
				}
			}
		} catch (e) {
			const msg = e instanceof Error ? e.message : String(e);
			state.statusMessage = `登录出错: ${msg}`;
			if (!isAuto) {
				toast.error(`登录出错: ${msg}`);
			}
		} finally {
			state.loggingIn = false;
		}
	}

	public async logout() {
		if (!state.loggedIn) {
			return;
		}

		try {
			const username =
				(state.userInfo as SrunLoginStateLoggedIn)?.user_name ||
				state.credentials.username;
			await apiLogout(username);
			state.manualLogout = true;
			state.loggedIn = false;
			state.userInfo = null;
			router.push("/");
			toast.success("已登出");
		} catch {
			toast.error("登出失败");
		}
	}
}

export const authManager = new AuthManager();
