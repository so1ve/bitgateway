import { reactive } from "vue";

import { getCredentials } from "./api/store";
import type { SrunLoginState } from "./types";

const credentials = await getCredentials();

export const state = reactive({
	firstOpen: true,
	manualLogout: false,
	initialized: false,
	initializeMessage: "",
	credentials,
	loggedIn: false,
	loggingIn: false,
	userInfo: null as SrunLoginState | null,
	statusMessage: "初始化中...",
});
