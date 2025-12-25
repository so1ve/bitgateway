<script setup lang="ts">
import { watch } from "vue";
import { Toaster } from "vue-sonner";

import OverlayScrollbars from "./components/OverlayScrollbars.vue";
import { authManager } from "./logic/auth";
import { state } from "./state";

import "vue-sonner/style.css";

watch(
	() => state.initialized,
	(val) => {
		if (val) {
			void authManager.start();
		}
	},
	{ immediate: true },
);
</script>

<template>
	<Toaster position="top-center" rich-colors />
	<div class="min-h-dvh min-w-dvw p-3">
		<RouterView v-if="state.initialized" v-slot="{ Component }">
			<template v-if="Component">
				<Transition mode="out-in">
					<Suspense>
						<component :is="Component" />

						<template #fallback>正在加载...</template>
					</Suspense>
				</Transition>
			</template>
		</RouterView>
		<div v-else class="flex h-full w-full items-center justify-center">
			<div v-if="!state.initializeMessage" class="animate-pulse text-gray-500">
				正在初始化...
			</div>
			<div v-else class="text-red-500">
				初始化失败：{{ state.initializeMessage }}
			</div>
		</div>
	</div>
	<OverlayScrollbars />
</template>
