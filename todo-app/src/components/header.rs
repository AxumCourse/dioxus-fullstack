use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx! {
        div { class: "flex justify-between items-center gap-x-2",
            div { class: "grow",
                input {
                    placeholder: "输入代办事项",
                    class: "block w-full px-3 py-1.5 outline-none ring-1 ring-inset ring-gray-300 focus:ring-2 focus:ring-blue-500",
                }
            }
            div { class: "shrink-0",
                button { class: "px-3 py-1.5 text-white bg-blue-500  hover:bg-blue-600",
                    "添加"
                }
            }

        }
    }
}
