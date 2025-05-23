use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct DialogProps {
    pub children: Element,
}

#[component]
pub fn Dialog(props: DialogProps) -> Element {
    rsx! {
        div { class: "fixed inset-0 z-[1000] bg-black/50 flex items-center justify-center",
            div { class: "bg-white rounded p-6 shadow-lg", {props.children} }
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct AlertProps {
    pub children: Element,
    pub onok: Option<EventHandler<MouseEvent>>,
}

#[component]
pub fn Alert(props: AlertProps) -> Element {
    rsx! {
        Dialog {
            div { class: "min-w-64 space-y-4",
                div { {props.children} }
                div { class: "flex justify-end items-center",
                    button {
                        class: "bg-gray-900 text-gray-50 px-3 py-1.5 text-sm rounded hover:bg-gray-800",
                        onclick: move |e| {
                            if let Some(onok) = props.onok {
                                onok.call(e);
                            }
                        },
                        "确定"
                    }
                }
            }
        }
    }
}
#[derive(PartialEq, Clone, Props)]
pub struct ConfirmProps {
    pub children: Element,
    pub onok: Option<EventHandler<MouseEvent>>,
    pub oncancel: Option<EventHandler<MouseEvent>>,
}

#[component]
pub fn Confirm(props: ConfirmProps) -> Element {
    rsx! {
        Dialog {
            div { class: "min-w-64 space-y-4",
                div { {props.children} }
                div { class: "flex justify-end items-center gap-x-2",
                    button {
                        class: "bg-gray-200 text-gray-900 px-3 py-1.5 text-sm rounded hover:bg-gray-300",
                        onclick: move |e| {
                            if let Some(oncancel) = props.oncancel {
                                oncancel.call(e);
                            }
                        },
                        "取消"
                    }
                    button {
                        class: "bg-gray-900 text-gray-50 px-3 py-1.5 text-sm rounded hover:bg-gray-800",
                        onclick: move |e| {
                            if let Some(onok) = props.onok {
                                onok.call(e);
                            }
                        },
                        "确定"
                    }
                }
            }
        }
    }
}
