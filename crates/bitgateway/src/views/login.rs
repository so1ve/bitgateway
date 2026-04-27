use dioxus::prelude::*;

use crate::auth::{self, SessionState};
use crate::components::{AppButton, ButtonVariant, PageTitle, ToastHandle};
use crate::config::{self, Credentials};
use crate::state::SessionPhase;

#[component]
pub fn LoginView(
    mut credentials: Signal<Credentials>,
    session_state: SessionState,
    toast: ToastHandle,
    on_settings: EventHandler<MouseEvent>,
) -> Element {
    let current = credentials.read().clone();
    let phase = session_state.phase();
    let busy = session_state.is_busy();
    let button_label = match phase {
        SessionPhase::LoggingIn { automatic: true } => "自动登录中...",
        SessionPhase::LoggingIn { automatic: false } => "登录中...",
        _ => "登录",
    };

    rsx! {
        section {
            class: "flex min-h-full flex-col gap-3.5",
            PageTitle {
                title: "登录",
                on_settings,
            }

            div {
                class: "grid gap-[7px]",
                label {
                    class: "text-xs font-medium text-[#444]",
                    r#for: "username",
                    "用户名 / 学号"
                }
                input {
                    id: "username",
                    class: "w-full min-w-0 border border-[#cfcfcf] bg-white px-2.5 py-[9px] text-[#222] outline-0 transition-colors duration-100 focus:border-[#777] rounded-[2px]",
                    autocomplete: "username",
                    disabled: busy,
                    initial_value: "{current.username}",
                    placeholder: "学号",
                    oninput: move |event| {
                        credentials.with_mut(|stored| stored.username = event.value());
                    },
                }

                label {
                    class: "text-xs font-medium text-[#444]",
                    r#for: "password",
                    "密码"
                }
                input {
                    id: "password",
                    class: "w-full min-w-0 border border-[#cfcfcf] bg-white px-2.5 py-[9px] text-[#222] outline-0 transition-colors duration-100 focus:border-[#777] rounded-[2px]",
                    r#type: "password",
                    autocomplete: "current-password",
                    disabled: busy,
                    initial_value: "{current.password}",
                    placeholder: "请输入统一认证密码",
                    oninput: move |event| {
                        credentials.with_mut(|stored| stored.password = event.value());
                    },
                }
            }

            div {
                class: "grid grid-cols-2 gap-2.5",
                label {
                    class: "flex min-w-0 cursor-pointer items-center gap-2 text-[13px] text-[#333]",
                    input {
                        r#type: "checkbox",
                        class: "accent-[#555]",
                        disabled: busy,
                        checked: current.remember_password,
                        onchange: move |event| {
                            let checked = event.checked();
                            credentials.with_mut(|stored| {
                                stored.remember_password = checked;
                                if !checked {
                                    stored.auto_login = false;
                                }
                            });
                            persist_preference(credentials);
                        },
                    }
                    span { "记住密码" }
                }

                label {
                    class: "flex min-w-0 cursor-pointer items-center gap-2 text-[13px] text-[#333]",
                    input {
                        r#type: "checkbox",
                        class: "accent-[#555]",
                        disabled: busy,
                        checked: current.auto_login,
                        onchange: move |event| {
                            let checked = event.checked();
                            credentials.with_mut(|stored| {
                                stored.auto_login = checked;
                                if checked {
                                    stored.remember_password = true;
                                }
                            });
                            persist_preference(credentials);
                        },
                    }
                    span { "自动登录" }
                }
            }

            AppButton {
                label: button_label,
                disabled: busy || !current.can_login(),
                variant: ButtonVariant::Primary,
                onclick: move |_| {
                    spawn(async move {
                        auth::submit_login(credentials, session_state, toast, false).await;
                    });
                },
            }
        }
    }
}

fn persist_preference(credentials: Signal<Credentials>) {
    let current = credentials.read().clone();
    config::credentials::save(&current);
}
