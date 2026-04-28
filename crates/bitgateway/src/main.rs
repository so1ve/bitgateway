#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod auth;
mod autostart;
mod components;
mod config;
mod network;
mod state;
mod tray;
mod update;
mod utils;
mod views;
mod windowing;

use dioxus::LaunchBuilder;
use dioxus::desktop::muda::Menu;
use dioxus::desktop::tao::dpi::LogicalSize;
use dioxus::desktop::tao::window::{Icon, WindowBuilder};
use dioxus::desktop::{Config, WindowCloseBehaviour, icon_from_memory};
use single_instance::SingleInstance;

const WINDOW_WIDTH: f64 = 280.0;
const WINDOW_HEIGHT: f64 = 380.0;
const APP_ICON: &[u8] = include_bytes!("../assets/icons/icon.png");

#[cfg(target_os = "macos")]
const SINGLE_INSTANCE_NAME: &str = "/tmp/io.mk1.bitgateway.lock";

#[cfg(not(target_os = "macos"))]
const SINGLE_INSTANCE_NAME: &str = "io.mk1.bitgateway";

fn main() {
    let instance = SingleInstance::new(SINGLE_INSTANCE_NAME).unwrap();
    if !instance.is_single() {
        return;
    }

    let start_silent = std::env::args().any(|arg| arg == "--silent");

    let window = WindowBuilder::new()
        .with_title("BITGATEWAY")
        .with_inner_size(LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
        .with_min_inner_size(LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
        .with_resizable(false)
        .with_maximizable(false)
        .with_visible(!start_silent);

    let config = Config::new()
        .with_window(window)
        .with_icon(icon_from_memory::<Icon>(APP_ICON).unwrap())
        .with_menu(None::<Menu>)
        .with_close_behaviour(WindowCloseBehaviour::WindowHides)
        .with_disable_context_menu(!cfg!(debug_assertions))
        .with_tray_icon_show_window_on_click(false);

    LaunchBuilder::desktop().with_cfg(config).launch(app::App);
}
