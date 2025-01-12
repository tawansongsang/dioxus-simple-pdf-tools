use dioxus::prelude::*;

use crate::routes::Route;

#[derive(PartialEq, Clone, Props)]
pub struct SidebarMenuProps {
    pub route: Route,
    pub name: String,
}

impl SidebarMenuProps {
    pub fn new(route: Route, name: &str) -> Self {
        Self {
            route,
            name: name.into(),
        }
    }
}

#[component]
pub fn SidebarMenu(props: SidebarMenuProps) -> Element {
    rsx! {
        li {
            Link {
                to: props.route,
                class: "m-1 block p-2 text-xs hover:rounded-md hover:bg-neutral-200 hover:text-sm hover:text-red-500 focus:rounded-md focus:bg-neutral-200 focus:text-sm focus:text-red-500 sm:text-sm sm:hover:text-base sm:focus:text-base md:text-base md:hover:text-lg md:focus:text-lg",
                "{props.name}"
            }
        }
    }
}
