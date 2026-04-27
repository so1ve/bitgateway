use dioxus::prelude::*;

use crate::auth::{self, SessionState};
use crate::components::{AppButton, ButtonVariant, PageTitle, ToastHandle};

#[component]
pub fn StatusView(
    session_state: SessionState,
    toast: ToastHandle,
    on_settings: EventHandler<MouseEvent>,
) -> Element {
    let info = session_state.online();

    if let Some(info) = info {
        let username = info.username.clone();

        return rsx! {
            section {
                class: "flex min-h-full flex-col gap-3.5",
                PageTitle {
                    title: "上网信息",
                    on_settings,
                }

                dl {
                    class: "mt-1 grid gap-0 border-t border-[#ddd]",
                    InfoRow { label: "用户名", value: info.username }
                    InfoRow { label: "IP", value: info.ip }
                    InfoRow { label: "已用流量", value: info.used_flux }
                    InfoRow { label: "已用时长", value: info.used_duration }
                    InfoRow { label: "账户余额", value: info.balance }
                }

                AppButton {
                    label: "注销登录",
                    disabled: session_state.is_busy(),
                    variant: ButtonVariant::DangerSecondary,
                    onclick: move |_| {
                        let username = username.clone();
                        spawn(async move {
                            auth::submit_logout(username, session_state, toast).await;
                        });
                    },
                }
            }
        };
    }

    rsx! {
        section {
            class: "flex min-h-full flex-col gap-3.5",
            PageTitle {
                title: "上网信息",
                on_settings,
            }
        }
    }
}

#[component]
fn InfoRow(label: &'static str, value: String) -> Element {
    rsx! {
        div {
            class: "grid grid-cols-[78px_minmax(0,1fr)] items-baseline gap-3 border-b border-[#ddd] py-2.5",
            dt {
                class: "text-xs font-medium text-[#666]",
                "{label}"
            }
            dd {
                class: "m-0 min-w-0 text-sm font-normal text-[#222] [overflow-wrap:anywhere]",
                "{value}"
            }
        }
    }
}
