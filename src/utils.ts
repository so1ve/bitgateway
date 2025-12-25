import humanize from "humanize-duration";

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
