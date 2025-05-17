use crate::layout::Frontend as FrontendLayout;
use crate::pages::{frontend::Home as FrontendHomePage, NotFound as NotFoundPage};
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/")]
        #[layout(FrontendLayout)]
            #[route("/")]
            FrontendHomePage {},
        #[end_layout]
    #[end_nest]
    
    #[route("/:..path")]
    NotFoundPage { path: Vec<String> },
}
