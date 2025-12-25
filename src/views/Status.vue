<script setup lang="ts">
import AppFooter from "../components/AppFooter.vue";
import { authManager } from "../logic/auth";
import { state } from "../state";
import type { SrunLoginStateLoggedIn } from "../types";
import { formatFlux, humanizeDuration } from "../utils";

async function handleLogout() {
	await authManager.logout();
}
</script>

<template>
	<div>
		<div v-if="state.userInfo && state.loggedIn" class="flex flex-col gap-4">
			<h2 class="text-2xl font-semibold text-center">登录状态</h2>
			<div class="card bg-gray-100/50 shadow-sm">
				<table class="table table-sm">
					<tbody>
						<tr>
							<td>用户名</td>
							<td>
								{{ (state.userInfo as SrunLoginStateLoggedIn).user_name }}
							</td>
						</tr>
						<tr>
							<td>IP</td>
							<td>{{ state.userInfo.online_ip }}</td>
						</tr>
						<tr>
							<td>已用流量</td>
							<td>
								{{
									formatFlux(
										(state.userInfo as SrunLoginStateLoggedIn).sum_bytes,
									)
								}}
							</td>
						</tr>
						<tr>
							<td>已用时长</td>
							<td>
								{{
									humanizeDuration(
										(state.userInfo as SrunLoginStateLoggedIn).sum_seconds *
											1000,
									)
								}}
							</td>
						</tr>
						<tr>
							<td>账户余额</td>
							<td>
								{{ (state.userInfo as SrunLoginStateLoggedIn).user_balance }}
							</td>
						</tr>
					</tbody>
				</table>
			</div>
			<button class="btn w-full btn-error" @click="handleLogout">注销</button>
		</div>
		<div v-else class="text-center mt-10">
			<span class="loading loading-spinner loading-lg" />
			<p class="mt-2">正在获取状态...</p>
		</div>
		<AppFooter />
	</div>
</template>
