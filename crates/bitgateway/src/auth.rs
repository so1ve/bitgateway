use std::sync::LazyLock;
use std::time::Duration;

use anyhow::Result as AnyhowResult;
use dioxus::prelude::*;
use tokio::sync::Mutex;

use crate::components::ToastHandle;
use crate::config::{self, Credentials};
use crate::network::{self, SessionSnapshot};
use crate::state::{OnlineInfo, SessionPhase};

const POLL_INTERVAL: Duration = Duration::from_secs(3);
const LOGIN_CONFIRM_ATTEMPTS: usize = 8;
const LOGIN_CONFIRM_DELAY: Duration = Duration::from_millis(500);
static AUTH_OPERATION_LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

#[derive(Clone, Copy, PartialEq)]
pub struct SessionState {
    phase: Signal<SessionPhase>,
    online: Signal<Option<OnlineInfo>>,
    manual_logout: Signal<bool>,
}

impl SessionState {
    pub const fn new(
        phase: Signal<SessionPhase>,
        online: Signal<Option<OnlineInfo>>,
        manual_logout: Signal<bool>,
    ) -> Self {
        Self {
            phase,
            online,
            manual_logout,
        }
    }

    pub fn phase(self) -> SessionPhase {
        (self.phase)()
    }

    pub fn online(self) -> Option<OnlineInfo> {
        self.online.read().clone()
    }

    pub fn is_busy(self) -> bool {
        self.phase().is_busy()
    }

    fn manual_logout(self) -> bool {
        (self.manual_logout)()
    }

    fn set_phase(self, phase: SessionPhase) {
        let mut current = self.phase;
        current.set(phase);
    }

    fn set_manual_logout(self, manual_logout: bool) {
        let mut current = self.manual_logout;
        current.set(manual_logout);
    }

    fn set_online(self, info: OnlineInfo) {
        let mut online = self.online;
        self.set_phase(SessionPhase::Online);
        online.set(Some(info));
        self.set_manual_logout(false);
    }

    fn set_offline(self) {
        let mut online = self.online;
        self.set_phase(SessionPhase::Offline);
        online.set(None);
    }

    fn set_logging_in(self, automatic: bool) {
        let mut online = self.online;
        self.set_phase(SessionPhase::LoggingIn { automatic });
        online.set(None);
    }
}

pub async fn run_status_loop(
    credentials: Signal<Credentials>,
    session_state: SessionState,
    toast: ToastHandle,
) {
    loop {
        let phase = session_state.phase();
        if matches!(
            phase,
            SessionPhase::LoggingIn { .. } | SessionPhase::LoggingOut
        ) {
            tokio::time::sleep(POLL_INTERVAL).await;

            continue;
        }

        refresh_status(credentials, session_state, toast).await;
        tokio::time::sleep(POLL_INTERVAL).await;
    }
}

pub async fn submit_login(
    credentials: Signal<Credentials>,
    session_state: SessionState,
    toast: ToastHandle,
    automatic: bool,
) {
    let attempt = credentials.read().clone();
    if !attempt.can_login() {
        session_state.set_offline();
        toast.show_error("请输入用户名和密码");

        return;
    }

    let _guard = AUTH_OPERATION_LOCK.lock().await;
    session_state.set_logging_in(automatic);

    let login_result = network::login(attempt.username.clone(), attempt.password.clone()).await;

    match login_result {
        Ok(()) => match wait_for_online_status().await {
            Ok(SessionSnapshot::Online(info)) => {
                config::credentials::save(&attempt);
                session_state.set_online(info);
                toast.show_success("登录成功");
            }
            Ok(SessionSnapshot::Offline(message)) => {
                session_state.set_offline();
                toast.show_error(format!("登录失败：{message}"));
            }
            Err(error) => {
                session_state.set_offline();
                toast.show_error(format!("登录失败：{error}"));
            }
        },
        Err(error) => match network::check_status().await {
            Ok(SessionSnapshot::Online(info)) => {
                config::credentials::save(&attempt);
                session_state.set_online(info);
                toast.show_success("登录成功");
            }
            Ok(SessionSnapshot::Offline(_)) | Err(_) => {
                session_state.set_offline();
                if !automatic {
                    toast.show_error(format!("登录失败：{error}"));
                }
            }
        },
    }
}

async fn wait_for_online_status() -> AnyhowResult<SessionSnapshot> {
    let mut result = network::check_status().await;
    if matches!(&result, Ok(SessionSnapshot::Online(_))) {
        return result;
    }

    for _ in 1..LOGIN_CONFIRM_ATTEMPTS {
        tokio::time::sleep(LOGIN_CONFIRM_DELAY).await;
        result = network::check_status().await;
        if matches!(&result, Ok(SessionSnapshot::Online(_))) {
            return result;
        }
    }

    result
}

pub async fn submit_logout(username: String, session_state: SessionState, toast: ToastHandle) {
    let _guard = AUTH_OPERATION_LOCK.lock().await;
    session_state.set_phase(SessionPhase::LoggingOut);
    session_state.set_manual_logout(true);

    let logout_result = network::logout(username).await;
    match logout_result {
        Ok(()) => {
            session_state.set_manual_logout(true);
            session_state.set_offline();
            toast.show_success("注销成功");
        }
        Err(error) => match network::check_status().await {
            Ok(SessionSnapshot::Online(info)) => {
                session_state.set_online(info);
                session_state.set_manual_logout(true);
                toast.show_error(format!("注销失败：{error}"));
            }
            Ok(SessionSnapshot::Offline(_)) | Err(_) => {
                session_state.set_manual_logout(true);
                session_state.set_offline();
                toast.show_success("注销成功");
            }
        },
    }
}

async fn refresh_status(
    credentials: Signal<Credentials>,
    session_state: SessionState,
    toast: ToastHandle,
) {
    let guard = AUTH_OPERATION_LOCK.lock().await;

    match network::check_status().await {
        Ok(SessionSnapshot::Online(info)) => session_state.set_online(info),
        Ok(SessionSnapshot::Offline(_)) | Err(_) => {
            let should_auto_login = {
                let creds = credentials.read();
                creds.auto_login && creds.can_login() && !session_state.manual_logout()
            };

            if should_auto_login {
                drop(guard);
                submit_login(credentials, session_state, toast, true).await;
            } else {
                session_state.set_offline();
            }
        }
    }
}
