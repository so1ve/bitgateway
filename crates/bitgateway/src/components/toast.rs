use std::time::Duration;

use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
enum ToastKind {
    Success,
    Error,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ToastMessage {
    id: u64,
    kind: ToastKind,
    text: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct ToastState {
    message: Option<ToastMessage>,
    next_id: u64,
}

#[derive(Clone, Copy, PartialEq)]
pub struct ToastHandle {
    state: Signal<ToastState>,
}

impl ToastHandle {
    fn current(self) -> Option<ToastMessage> {
        self.state.read().message.clone()
    }

    fn show(self, kind: ToastKind, text: impl Into<String>) {
        let mut state = self.state;
        state.with_mut(|state| {
            state.next_id = state.next_id.saturating_add(1);
            state.message = Some(ToastMessage {
                id: state.next_id,
                kind,
                text: text.into(),
            });
        });
    }

    pub fn show_success(self, text: impl Into<String>) {
        self.show(ToastKind::Success, text);
    }

    pub fn show_error(self, text: impl Into<String>) {
        self.show(ToastKind::Error, text);
    }

    fn clear(self, id: u64) {
        let mut state = self.state;
        state.with_mut(|state| {
            if state.message.as_ref().is_some_and(|toast| toast.id == id) {
                state.message = None;
            }
        });
    }
}

pub fn use_provide_toast() -> ToastHandle {
    use_context_provider(|| ToastHandle {
        state: Signal::new(ToastState::default()),
    })
}

pub fn use_toast() -> ToastHandle {
    use_context::<ToastHandle>()
}

#[component]
pub fn Toast() -> Element {
    let handle = use_toast();
    let toast = handle.current();
    let mut scheduled_toast_id = use_signal(|| None::<u64>);

    use_effect(move || {
        let Some(toast_id) = handle.current().as_ref().map(|toast| toast.id) else {
            return;
        };

        if scheduled_toast_id() == Some(toast_id) {
            return;
        }

        scheduled_toast_id.set(Some(toast_id));
        spawn(async move {
            tokio::time::sleep(Duration::from_millis(3_600)).await;
            handle.clear(toast_id);
        });
    });

    if let Some(toast) = toast {
        let tone_class = match toast.kind {
            ToastKind::Success => "border-[#b7c7b7] bg-[#f3f7f3]",
            ToastKind::Error => "border-[#d8b8b8] bg-[#f8f1f1]",
        };
        let class = format!(
            "absolute right-3 top-12 z-20 w-[260px] max-w-[calc(100%-24px)] border px-2.5 py-2 text-xs leading-[1.45] text-[#222] {tone_class}"
        );

        return rsx! {
            div {
                key: "{toast.id}",
                class,
                role: "status",
                "{toast.text}"
            }
        };
    }

    rsx! {}
}
