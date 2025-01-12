use dioxus::prelude::*;

use crate::components::BaseLayout;
use crate::views::{Home, Merge, NotFound, Split};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(BaseLayout)]
    #[route("/")]
    Home {},
    #[route("/merge")]
    Merge {},
    #[route("/split")]
    Split {},
    // PageNotFound is a catch all route that will match any route and placing the matched segments in the route field
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}
