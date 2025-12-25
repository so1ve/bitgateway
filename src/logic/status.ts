import { invoke } from "@tauri-apps/api/core";

import { DEFAULT_INTERVAL } from "../constants";
import type { ApiResponse, SrunLoginState } from "../types";

export type StatusResult =
	| { type: "online"; data: SrunLoginState }
	| { type: "offline"; data: SrunLoginState }
	| { type: "error"; error: string };

export async function* monitorStatus(
	intervalMs: number = DEFAULT_INTERVAL,
): AsyncGenerator<StatusResult> {
	while (true) {
		try {
			const response =
				await invoke<ApiResponse<SrunLoginState, string>>("check_status");
			if (response.success) {
				yield response.data.error === "ok"
					? { type: "online", data: response.data }
					: { type: "offline", data: response.data };
			} else {
				yield { type: "error", error: response.error };
			}
		} catch (e) {
			yield {
				type: "error",
				error: e instanceof Error ? e.message : String(e),
			};
		}

		await new Promise((resolve) => setTimeout(resolve, intervalMs));
	}
}
