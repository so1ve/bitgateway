import { init as apiInit } from "../api/methods";
import { DEFAULT_INTERVAL } from "../constants";
import { state } from "../state";

class InitManager {
	private isRunning = false;
	private retryTimer: ReturnType<typeof setTimeout> | null = null;

	public async start() {
		if (this.isRunning || state.initialized) {
			return;
		}
		this.isRunning = true;

		await this.attempt();
	}

	private async attempt() {
		if (state.initialized) {
			return;
		}

		try {
			const response = await apiInit();
			if (response.success) {
				state.initialized = true;
				state.initializeMessage = "";
				this.isRunning = false;
			} else {
				state.initializeMessage = response.error;
				this.scheduleRetry();
			}
		} catch (e) {
			state.initializeMessage = e instanceof Error ? e.message : String(e);
			this.scheduleRetry();
		}
	}

	private scheduleRetry() {
		if (this.retryTimer) {
			clearTimeout(this.retryTimer);
		}

		this.retryTimer = setTimeout(() => {
			this.retryTimer = null;
			void this.attempt();
		}, DEFAULT_INTERVAL);
	}
}

export const initManager = new InitManager();
