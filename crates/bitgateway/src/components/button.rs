use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,
    DangerSecondary,
}

#[component]
pub fn AppButton(
    label: String,
    disabled: bool,
    variant: ButtonVariant,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let class = match variant {
        ButtonVariant::Primary => {
            "w-full cursor-pointer border border-[#333] bg-[#333] px-3 py-2.5 font-medium text-white transition-colors duration-100 hover:border-[#444] hover:bg-[#444] disabled:cursor-not-allowed disabled:opacity-55 rounded-[2px]"
        }
        ButtonVariant::DangerSecondary => {
            "mt-auto w-full cursor-pointer border border-[#bbb] bg-transparent px-3 py-2.5 font-medium text-[#333] transition-colors duration-100 hover:border-[#b9a0a0] hover:bg-[#f4eeee] hover:text-[#6f2222] disabled:cursor-not-allowed disabled:opacity-55 rounded-[2px]"
        }
    };

    rsx! {
        button {
            class,
            disabled,
            onclick,
            "{label}"
        }
    }
}
