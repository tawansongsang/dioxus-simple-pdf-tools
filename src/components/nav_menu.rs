use dioxus::prelude::*;

use crate::routes::Route;

#[derive(PartialEq, Clone, Props)]
pub struct NavMenuProps {
    pub route: Route,
    pub name: String,
}

impl NavMenuProps {
    pub fn new(route: Route, name: &str) -> Self {
        Self {
            route,
            name: name.into(),
        }
    }
}

#[component]
pub fn NavMenu(props: NavMenuProps) -> Element {
    rsx! {
        li { class: "ml-1 mr-1",
            Link {
                to: props.route,
                class: "p-1 text-center text-xs hover:text-sm hover:text-red-500 focus:text-sm focus:text-red-500 sm:text-sm sm:hover:text-base sm:focus:text-base md:text-base md:hover:text-lg md:focus:text-lg",
                active_class: "text-red-500",
                "{props.name}"
            }
        }
    }
}
