use dioxus::prelude::*;

#[component]
pub fn Upload() -> Element {
    rsx! {
        div { class: "space-y-6",
            div { class: "bg-white/70 border border-inset border-dashed border-gray-400 h-64 w-full hover:border-purple-400",
                label {
                    r#for: "upload",
                    class: "cursor-pointer block w-full h-full flex justify-center items-center",
                    "上传图片"
                }
                input {
                    id: "upload",
                    r#type: "file",
                    accept: "image/*",
                    class: "hidden",
                }
            }

            ul { class: "grid grid-cols-2 gap-4 lg:grid-cols-5",
                for i in 0..12 {
                    li {
                        key: "{i}",
                        class: "h-48 w-full lg:h-48 overflow-hidden border border-gray-400 focus-within:border-purple-400 rounded-md shadow-sm",
                        img {
                            src: "https://file.axum.eu.org/wepay-chat/1.png",
                            class: "h-40 w-full object-cover",
                        }
                        div { class: "h-8 w-full",
                            input {
                                class: "block w-full p-1 outline-none text-sm text-gray-600 focus:text-inherit",
                                value: "https://file.axum.eu.org/wepay-chat/1.png",
                            }
                        }
                    }
                }
            

            }
        }
    }
}
