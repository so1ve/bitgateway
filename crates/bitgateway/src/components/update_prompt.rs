use dioxus::prelude::*;

use crate::update::UpdateInfo;

#[component]
pub fn UpdatePrompt(info: UpdateInfo, on_dismiss: EventHandler<MouseEvent>) -> Element {
    rsx! {
        aside {
            class: "absolute bottom-10 left-3 right-3 z-30 border border-[#c9c2aa] bg-[#fffaf0] p-3 text-left text-xs leading-[1.45] text-[#2d2a22] shadow-[0_8px_24px_rgba(0,0,0,0.10)]",
            role: "status",
            div {
                class: "mb-2 flex items-start justify-between gap-3",
                div {
                    class: "min-w-0",
                    p {
                        class: "m-0 font-semibold text-[#222]",
                        "发现新版本 v{info.version}"
                    }
                    p {
                        class: "m-0 mt-1 text-[#665f4b]",
                        "当前版本较旧，可前往 GitHub Releases 下载最新版。"
                    }
                }
                button {
                    class: "-mr-1 -mt-1 cursor-pointer border-0 bg-transparent px-1 text-lg leading-none text-[#665f4b] hover:text-[#222]",
                    title: "稍后提醒",
                    onclick: move |event| on_dismiss.call(event),
                    "×"
                }
            }
            div {
                class: "flex items-center gap-2",
                a {
                    href: info.html_url.clone(),
                    target: "_blank",
                    class: "inline-flex flex-1 items-center justify-center rounded-[2px] border border-[#333] bg-[#333] px-3 py-2 font-medium text-white no-underline transition-colors duration-100 hover:border-[#444] hover:bg-[#444]",
                    "打开下载页"
                }
                button {
                    class: "inline-flex cursor-pointer items-center justify-center rounded-[2px] border border-[#c8bea2] bg-transparent px-3 py-2 font-medium text-[#4d4635] transition-colors duration-100 hover:border-[#a89562] hover:bg-[#fff4d7]",
                    onclick: move |event| on_dismiss.call(event),
                    "稍后"
                }
            }
        }
    }
}
