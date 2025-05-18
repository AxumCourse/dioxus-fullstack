use crate::{
    components::{BookIcon, DashboardIcon, GithubIcon},
    Route,
};
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
                    li { class: "relative group",
                        a {
                            href: "https://axum.eu.org/subject/dioxus-fullstack",
                            target: "_blank",
                            BookIcon {}
                        }
                        div { class: "hidden absolute top-6 min-w-max px-2 py-1 left-1/2 -translate-x-1/2 text-xs bg-red-600 z-[1] lg:block opacity-0 invisible group-hover:opacity-100 group-hover:visible bg-white/70 rounded",
                            "教程"
                        }
                    }
                    li { class: "relative group",
                        a {
                            href: "https://github.com/AxumCourse/dioxus-fullstack",
                            target: "_blank",
                            GithubIcon {}
                        }
                        div { class: "hidden absolute top-6 min-w-max px-2 py-1 left-1/2 -translate-x-1/2 text-xs bg-red-600 z-[1] lg:block opacity-0 invisible group-hover:opacity-100 group-hover:visible bg-white/70 rounded",
                            "源码"
                        }
                    }
                    li { class: "relative group",
                        Link { to: Route::FrontendHomePage {}, DashboardIcon {} }
                        div { class: "hidden absolute top-6 min-w-max px-2 py-1 left-1/2 -translate-x-1/2 text-xs bg-red-600 z-[1] lg:block opacity-0 invisible group-hover:opacity-100 group-hover:visible bg-white/70 rounded",
                            "后台"
                        }
                    }
                }
            }
        }
        main { class: "axum-container p-4 relative", Outlet::<Route> {} }
    }
}
