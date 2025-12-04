export type ApiResponse<T, E = null> =
	| { success: true; data: T }
	| { success: false; error: E };

export type SrunLoginState<
	T = SrunLoginStateLoggedIn | SrunLoginStateLoggedOut,
> = {
	error: string;
	online_ip: string;
} & T;

export interface SrunLoginStateLoggedIn {
	ServerFlag: number;
	add_time: number;
	all_bytes: number;
	bytes_in: number;
	bytes_out: number;
	checkout_date: number;
	domain: string;
	group_id: string;
	keepalive_time: number;
	products_name: string;
	real_name: string;
	remain_bytes: number;
	remain_seconds: number;
	sum_bytes: number;
	sum_seconds: number;
	sysver: string;
	user_balance: number;
	user_charge: number;
	user_mac: string;
	user_name: string;
	wallet_balance: number;
}

export interface SrunLoginStateLoggedOut {
	client_ip: string;
	error_msg: string;
	res: string;
	srun_ver: string;
	st: number;
}

export interface Credentials {
	username: string;
	password: string;
	rememberMe: boolean;
	autoLogin: boolean;
	autoReconnect: boolean;
}
