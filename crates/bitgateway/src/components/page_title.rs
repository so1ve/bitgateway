use dioxus::prelude::*;

#[component]
pub fn PageTitle(title: &'static str, on_settings: EventHandler<MouseEvent>) -> Element {
    rsx! {
        div {
            class: "flex items-center justify-between pt-1",
            h1 {
                class: "m-0 text-[28px] font-semibold leading-[1.2] tracking-normal text-[#222]",
                "{title}"
            }
            button {
                class: "cursor-pointer rounded-[2px] border border-[#d8d8d8] bg-white px-2 py-1 text-xs font-normal text-[#555] transition-colors duration-100 hover:border-[#bbb] hover:text-[#222]",
                onclick: on_settings,
                "设置"
            }
        }
    }
}
