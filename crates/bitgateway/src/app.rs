use dioxus::prelude::*;

use crate::auth::SessionState;
use crate::components::{AppFooter, Toast, UpdatePrompt, use_provide_toast};
use crate::config::{credentials, settings};
use crate::state::SessionPhase;
use crate::update::UpdateInfo;
use crate::views::{LoginView, SettingsPanel, StatusView};
use crate::{auth, tray, update, windowing};

const TAILWIND_CSS: &str = include_str!("../assets/tailwind.css");

#[component]
pub fn App() -> Element {
    let credentials = use_signal(credentials::load);
    let settings = use_signal(settings::load);
    let phase = use_signal(SessionPhase::default);
    let online = use_signal(|| None);
    let manual_logout = use_signal(|| false);
    let session_state = SessionState::new(phase, online, manual_logout);
    let toast = use_provide_toast();
    let mut settings_open = use_signal(|| false);
    let mut update_info = use_signal(|| None::<UpdateInfo>);
    let version = env!("CARGO_PKG_VERSION");

    tray::use_tray();
    windowing::use_center_window();

    use_future(move || async move {
        auth::run_status_loop(credentials, session_state, toast).await;
    });

    use_future(move || async move {
        if let Ok(Some(info)) = update::check_for_update().await {
            update_info.set(Some(info));
        }
    });

    rsx! {
        style { {TAILWIND_CSS} }
        main {
            class: "relative flex h-screen w-screen min-w-0 flex-col overflow-hidden bg-[#f5f5f5] font-sans text-[#222]",
            div {
                class: "min-h-0 flex-1 px-[18px] pb-11 pt-2",
                {
                    match phase() {
                        SessionPhase::Online | SessionPhase::LoggingOut => rsx! {
                            StatusView {
                                session_state,
                                toast,
                                on_settings: move |_| settings_open.set(true),
                            }
                        },
                        _ => rsx! {
                            LoginView {
                                credentials,
                                session_state,
                                toast,
                                on_settings: move |_| settings_open.set(true),
                            }
                        },
                    }
                }
            }

            AppFooter { version }

            Toast {}

            if let Some(info) = update_info() {
                UpdatePrompt {
                    info,
                    on_dismiss: move |_| update_info.set(None),
                }
            }

            if settings_open() {
                SettingsPanel {
                    settings,
                    settings_open,
                    toast,
                }
            }
        }
    }
}
