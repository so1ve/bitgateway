use dioxus::desktop::tao::dpi::PhysicalPosition;
use dioxus::desktop::window;
use dioxus::prelude::*;

pub fn use_center_window() {
    use_hook(center_window);
}

fn center_window() {
    let desktop = window();
    let Some(monitor) = desktop
        .current_monitor()
        .or_else(|| desktop.primary_monitor())
    else {
        return;
    };

    let monitor_position = monitor.position();
    let monitor_size = monitor.size();
    let window_size = desktop.outer_size();
    let x = monitor_position.x + (monitor_size.width as i32 - window_size.width as i32) / 2;
    let y = monitor_position.y + (monitor_size.height as i32 - window_size.height as i32) / 2;

    desktop.set_outer_position(PhysicalPosition::new(x, y));
}
