use crate::Route;
use dioxus::prelude::*;

const LOGO_IMG: Asset = asset!("/assets/logo.png");

#[component]
pub fn Frontend() -> Element {
    rsx! {
        header { class: "flex justify-between items-center p-4 axum-container",
            section {
                Link {
                    class: "flex justify-start items-center gap-x-2",
                    to: Route::FrontendHomePage {},
                    img {
                        src: LOGO_IMG,
                        alt: "AXUM中文网图床",
                        class: "w-6 lg:w-8 object-cover",
                    }
                    h2 { class: "text-lg lg:text-xl", "AXUM中文网图床" }
                }
            }
            nav {
                ul { class: "flex justify-end items-center gap-x-2",
                    li {
                        a { href: "https://github.com", target: "_blank", "github" }
                    
                    }
                    li {

                        Link { to: Route::FrontendHomePage {}, "后台管理" }
                    }
                }
            }
        }
        main { class: "axum-container p-4 relative", Outlet::<Route> {} }
    }
}
