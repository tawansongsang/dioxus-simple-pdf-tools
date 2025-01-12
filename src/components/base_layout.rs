use dioxus::prelude::*;

use crate::{
    components::{Footer, Navbar},
    routes::Route,
};

#[component]
pub fn BaseLayout() -> Element {
    rsx! {
        Navbar {}
        Outlet::<Route> {}
        Footer {}
    }
}
