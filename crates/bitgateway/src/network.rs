use std::future::Future;
use std::sync::LazyLock;
use std::time::Duration;

use anyhow::{Context, Result, bail};
use bitgateway_client::{Client, LoginState, PortalResponse};

use crate::state::OnlineInfo;
use crate::utils::{format_balance, format_flux, humanize_duration};

const CHECK_TIMEOUT: Duration = Duration::from_secs(8);
const LOGIN_TIMEOUT: Duration = Duration::from_secs(10);
const LOGOUT_TIMEOUT: Duration = Duration::from_secs(10);
static CLIENT: LazyLock<Client> = LazyLock::new(Client::new);

#[derive(Clone, Debug, PartialEq)]
pub enum SessionSnapshot {
    Online(OnlineInfo),
    Offline(String),
}

pub async fn check_status() -> Result<SessionSnapshot> {
    let client = &*CLIENT;
    let state = with_timeout(CHECK_TIMEOUT, "登录状态检查超时", async move {
        client.get_login_state().await.context("获取登录状态失败")
    })
    .await?;

    Ok(state.into())
}

pub async fn login(username: String, password: String) -> Result<()> {
    let client = &*CLIENT;
    let response = with_timeout(LOGIN_TIMEOUT, "登录请求超时", async move {
        client
            .login(&username, &password)
            .await
            .context("登录请求失败")
    })
    .await?;

    if portal_success(&response) {
        return Ok(());
    }

    bail!(portal_error_message(&response, "登录失败"));
}

pub async fn logout(username: String) -> Result<()> {
    let client = &*CLIENT;
    let response = with_timeout(LOGOUT_TIMEOUT, "注销请求超时", async move {
        client.logout(&username).await.context("注销请求失败")
    })
    .await?;

    if portal_success(&response) {
        return Ok(());
    }

    bail!(portal_error_message(&response, "注销失败"));
}

async fn with_timeout<T>(
    duration: Duration,
    message: &'static str,
    future: impl Future<Output = Result<T>>,
) -> Result<T> {
    tokio::time::timeout(duration, future)
        .await
        .context(message)?
}

impl From<LoginState> for SessionSnapshot {
    fn from(state: LoginState) -> Self {
        let success =
            state.error == "ok" || state.user_name.is_some() || state.sum_seconds.is_some();
        if success {
            return Self::Online(OnlineInfo {
                username: state.user_name.unwrap_or("未知用户".to_string()),
                ip: state.online_ip.to_string(),
                used_flux: format_flux(state.sum_bytes.unwrap_or(0)),
                used_duration: humanize_duration(state.sum_seconds.unwrap_or(0)),
                balance: state
                    .user_balance
                    .map(format_balance)
                    .unwrap_or("-".to_string()),
            });
        }

        let message = state
            .error_msg
            .filter(|message| !message.trim().is_empty())
            .unwrap_or("当前未登录".to_string());

        Self::Offline(message)
    }
}

fn portal_success(response: &PortalResponse) -> bool {
    response.error == "ok" || response.suc_msg.is_some()
}

fn portal_error_message(response: &PortalResponse, fallback: &str) -> String {
    if !response.error_msg.trim().is_empty() {
        return response.error_msg.clone();
    }
    if !response.error.trim().is_empty() {
        return response.error.clone();
    }

    fallback.to_string()
}
