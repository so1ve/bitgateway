<script setup lang="ts">
import { watch } from "vue";

import AppFooter from "../components/AppFooter.vue";
import { authManager } from "../logic/auth";
import { state } from "../state";

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

async function handleLogin() {
	await authManager.login(false);
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
						:disabled="state.loggingIn"
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
						:disabled="state.loggingIn"
						type="password"
					/>
				</div>

				<fieldset class="grid grid-cols-2">
					<label class="label cursor-pointer">
						<input
							v-model="state.credentials.rememberMe"
							class="checkbox"
							:disabled="state.loggingIn"
							type="checkbox"
						/>
						<span class="label-text ml-2">记住我</span>
					</label>
					<label class="label cursor-pointer">
						<input
							v-model="state.credentials.autoLogin"
							class="checkbox"
							:disabled="state.loggingIn"
							type="checkbox"
						/>
						<span class="label-text ml-2">自动登录</span>
					</label>
				</fieldset>

				<div
					v-if="state.statusMessage"
					class="text-sm text-center text-gray-500 mb-2"
				>
					{{ state.statusMessage }}
				</div>

				<button
					class="btn w-full"
					:class="{
						'btn-accent': !state.loggingIn,
						'btn-disabled': state.loggingIn,
					}"
					:disabled="state.loggingIn"
					type="submit"
				>
					<span v-if="state.loggingIn" class="loading loading-spinner" />
					<span v-if="state.loggingIn && state.retryCount > 0">
						重试中 ({{ state.retryCount }})
					</span>
					<span v-else-if="state.loggingIn">登录中...</span>
					<span v-else>登录</span>
				</button>
			</form>
		</div>
		<AppFooter />
	</div>
</template>
