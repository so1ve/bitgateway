use dioxus::prelude::*;

use crate::components::ToastHandle;
use crate::config::Settings;
use crate::{autostart, config};

#[component]
pub fn SettingsPanel(
    settings: Signal<Settings>,
    mut settings_open: Signal<bool>,
    toast: ToastHandle,
) -> Element {
    let current = settings.read().clone();
    let silent_class = if current.auto_start {
        "flex min-w-0 cursor-pointer items-start gap-2 border-t border-[#e1e1e1] py-3 text-[13px] text-[#333]"
    } else {
        "flex min-w-0 cursor-not-allowed items-start gap-2 border-t border-[#e1e1e1] py-3 text-[13px] text-[#333] opacity-60"
    };

    rsx! {
        div {
            class: "absolute inset-0 grid place-items-center bg-black/20 p-[18px]",
            div {
                class: "w-full border border-[#d0d0d0] bg-white p-4",
                div {
                    class: "mb-3.5 flex items-start justify-between gap-3",
                    div {
                        p {
                            class: "mb-1.5 mt-0 text-xs font-medium text-[#666]",
                            "设置"
                        }
                        h2 {
                            class: "m-0 text-xl font-semibold tracking-normal text-[#222]",
                            "启动行为"
                        }
                    }
                    button {
                        class: "cursor-pointer border-0 bg-transparent text-[22px] leading-none text-[#444] hover:text-[#111]",
                        onclick: move |_| settings_open.set(false),
                        "×"
                    }
                }

                label {
                    class: "flex min-w-0 cursor-pointer items-start gap-2 border-t border-[#e1e1e1] py-3 text-[13px] text-[#333]",
                    input {
                        r#type: "checkbox",
                        class: "accent-[#555]",
                        checked: current.auto_start,
                        onchange: move |event| {
                            let checked = event.checked();
                            update_settings(settings, toast, |next| {
                                next.auto_start = checked;
                                if !checked {
                                    next.silent_start = false;
                                }
                            });
                        },
                    }
                    span {
                        class: "grid gap-1",
                        strong {
                            class: "font-medium",
                            "自动启动"
                        }
                        small {
                            class: "leading-[1.35] text-[#666]",
                                "开机时自动运行 BITGATEWAY"
                        }
                    }
                }

                label {
                    class: "{silent_class}",
                    input {
                        r#type: "checkbox",
                        class: "accent-[#555]",
                        disabled: !current.auto_start,
                        checked: current.silent_start,
                        onchange: move |event| {
                            let checked = event.checked();
                            update_settings(settings, toast, |next| next.silent_start = checked);
                        },
                    }
                    span {
                        class: "grid gap-1",
                        strong {
                            class: "font-medium",
                            "静默启动"
                        }
                        small {
                            class: "leading-[1.35] text-[#666]",
                            "随开机启动时隐藏窗口，仅保留托盘"
                        }
                    }
                }
            }
        }
    }
}

fn update_settings(
    mut settings: Signal<Settings>,
    toast: ToastHandle,
    update: impl FnOnce(&mut Settings),
) {
    let previous = settings.read().clone();
    let mut next = previous.clone();
    update(&mut next);

    match autostart::sync(&next) {
        Ok(()) => {
            config::settings::save(&next);
            settings.set(next);
            toast.show_success("设置已保存");
        }
        Err(error) => {
            settings.set(previous);
            toast.show_error(format!("保存设置失败：{error}"));
        }
    }
}
