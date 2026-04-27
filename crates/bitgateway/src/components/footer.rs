use dioxus::prelude::*;

const REPOSITORY_URL: &str = "https://github.com/so1ve/bitgateway";

#[component]
pub fn AppFooter(version: String) -> Element {
    rsx! {
        footer {
            class: "absolute bottom-0 left-0 right-0 border-t border-[#ddd] bg-[#f7f7f7] text-center text-[11px] font-medium tracking-[0.04em]",
            a {
                href: REPOSITORY_URL,
                class: "block px-3 py-2 text-[#666] no-underline transition-colors duration-100 hover:text-[#222] focus-visible:outline focus-visible:outline-1 focus-visible:outline-offset-[-2px] focus-visible:outline-[#777]",
                title: "打开 so1ve/bitgateway 项目仓库",
                "BITGATEWAY v{version}"
            }
        }
    }
}
