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

const triggerCheckStatus = await useCheckStatus(async (loggedIn) => {
	if (loggedIn) {
		router.push("/status");
	}
});

watch(
	() => state.credentials.rememberMe,
	(newVal) => {
		if (newVal === false) {
			if (state.credentials.autoLogin === true) {
				state.credentials.autoLogin = false;
			}
			if (state.credentials.autoReconnect === true) {
				state.credentials.autoReconnect = false;
			}
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

watch(
	() => state.credentials.autoReconnect,
	(newVal) => {
		if (newVal === true && state.credentials.rememberMe === false) {
			state.credentials.rememberMe = true;
		}
	},
);

if (state.credentials.autoLogin && (state.firstOpen || !state.manualLogout)) {
	state.firstOpen = false;

	await login(state.credentials);
	await triggerCheckStatus();
} else {
	loading.value = false;
}

async function handleLogin() {
	loading.value = true;
	if (state.credentials.rememberMe) {
		await setCredentials(state.credentials);
	}
	const response = await login(state.credentials);
	if (response.success) {
		await triggerCheckStatus();
		toast.success("登录成功！");
	} else {
		loading.value = false;
		toast.error(`登录失败：${response.error}`);
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

				<fieldset class="grid grid-cols-3">
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
					<label class="label">
						<input
							v-model="state.credentials.autoReconnect"
							class="checkbox"
							type="checkbox"
							:disabled="!state.credentials.rememberMe"
						/>
						自动重连
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
					登录
				</button>
			</form>
		</div>
		<AppFooter />
	</div>
</template>
