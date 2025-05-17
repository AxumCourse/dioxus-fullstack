use crate::pages::{frontend::Home as FrontendHomePage, NotFound as NotFoundPage};
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
//#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    FrontendHomePage {},

    #[route("/:..path")]
    NotFoundPage { path: Vec<String> },
}
