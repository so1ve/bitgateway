<script setup lang="ts">
import { ref, watch } from "vue";
import { useRouter } from "vue-router";
import { toast } from "vue-sonner";

import { login, setCredentials } from "../api";
import AppFooter from "../components/AppFooter.vue";
import { state } from "../state";
import { useCheckStatus } from "../utils";

const router = useRouter();
const loading = ref(true);
const retryCount = ref(0);
const isRetrying = ref(false);

// 重试机制配置
const RETRY_CONFIG = {
	maxRetries: 10,
	retryInterval: 5000,
	timeout: 50_000,
};

const triggerCheckStatus = await useCheckStatus(async (loggedIn) => {
	if (loggedIn) {
		router.push("/status");
	}
});

watch(
	() => state.credentials.rememberMe,
	(newVal) => {
		if (newVal === false && state.credentials.autoLogin === true) {
			state.credentials.autoLogin = false;
		}
	},
);

watch(
	() => state.credentials.autoLogin,
	(newVal) => {
		if (newVal === true && state.credentials.rememberMe === false) {
			state.credentials.rememberMe = true;
		}
	},
);

async function autoLoginWithRetry() {
	state.firstOpen = false;

	async function attemptLogin(): Promise<boolean> {
		try {
			const loginPromise = login(state.credentials);
			const timeoutPromise = new Promise<never>((_resolve, reject) => {
				setTimeout(() => reject(new Error("登录超时")), RETRY_CONFIG.timeout);
			});

			const response = await Promise.race([loginPromise, timeoutPromise]);

			if (response.success) {
				await triggerCheckStatus();

				return true;
			} else {
				console.error(
					`登录尝试失败 (第${retryCount.value + 1}次):`,
					response.error,
				);

				return false;
			}
		} catch (error) {
			console.error(`登录尝试失败 (第${retryCount.value + 1}次):`, error);

			return false;
		}
	}

	const loginSuccess = await attemptLogin();

	if (loginSuccess) {
		loading.value = false;

		return;
	}

	isRetrying.value = true;

	for (let i = 0; i < RETRY_CONFIG.maxRetries; i++) {
		retryCount.value = i + 1;
		toast.info(
			`登录失败，正在重试 (${retryCount.value}/${RETRY_CONFIG.maxRetries})...`,
		);

		// 等待重试间隔
		await new Promise((resolve) =>
			setTimeout(resolve, RETRY_CONFIG.retryInterval),
		);

		const success = await attemptLogin();
		if (success) {
			isRetrying.value = false;
			loading.value = false;

			return;
		}
	}

	isRetrying.value = false;
	loading.value = false;
	toast.error(`自动登录失败，已重试${RETRY_CONFIG.maxRetries}次，请手动登录`);
}

if (state.credentials.autoLogin && (state.firstOpen || !state.manualLogout)) {
	await autoLoginWithRetry();
} else {
	loading.value = false;
}

async function handleLogin() {
	loading.value = true;
	retryCount.value = 0;
	isRetrying.value = false;

	if (state.credentials.rememberMe) {
		await setCredentials(state.credentials);
	}

	try {
		const loginPromise = login(state.credentials);
		const timeoutPromise = new Promise<never>((_resolve, reject) => {
			setTimeout(() => reject(new Error("登录超时")), RETRY_CONFIG.timeout);
		});

		const response = (await Promise.race([
			loginPromise,
			timeoutPromise,
		])) as any;

		if (response.success) {
			await triggerCheckStatus();
			toast.success("登录成功！");
		} else {
			loading.value = false;
			toast.error(`登录失败：${response.error}`);
		}
	} catch (error) {
		loading.value = false;
		toast.error(
			`登录失败：${error instanceof Error ? error.message : "未知错误"}`,
		);
	}
}
</script>

<template>
	<div>
		<div>
			<h2 class="text-2xl font-semibold mb-2 text-center">登录</h2>
			<form class="flex flex-col gap-4" @submit.prevent="handleLogin">
				<div class="form-control w-full">
					<label class="label">
						<span class="label-text text-base-content">用户名/学号</span>
					</label>
					<input
						v-model="state.credentials.username"
						class="input input-bordered w-full"
						type="text"
					/>
				</div>

				<div class="form-control w-full">
					<label class="label">
						<span class="label-text text-base-content">密码</span>
					</label>
					<input
						v-model="state.credentials.password"
						class="input input-bordered w-full"
						type="password"
					/>
				</div>

				<fieldset class="grid grid-cols-2">
					<label class="label">
						<input
							v-model="state.credentials.rememberMe"
							class="checkbox"
							type="checkbox"
						/>
						记住我
					</label>
					<label class="label">
						<input
							v-model="state.credentials.autoLogin"
							class="checkbox"
							type="checkbox"
						/>
						自动登录
					</label>
				</fieldset>

				<button
					class="btn w-full"
					:class="{
						'btn-accent': !loading,
						'btn-disabled': loading,
					}"
					:disabled="loading"
					type="submit"
				>
					<span v-if="loading" class="loading loading-spinner" />
					<span v-if="isRetrying">
						重试中 ({{ retryCount }}/{{ RETRY_CONFIG.maxRetries }})
					</span>
					<span v-else>登录</span>
				</button>
			</form>
		</div>
		<AppFooter />
	</div>
</template>
