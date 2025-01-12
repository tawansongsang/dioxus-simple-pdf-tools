use dioxus::prelude::*;

use crate::routes::Route;

#[derive(PartialEq, Clone, Props)]
pub struct CardProps {
    pub route: Route,
    pub img: Asset,
    pub title: String,
    pub desc: String,
}

impl CardProps {
    pub fn new(route: Route, img: Asset, title: &str, desc: &str) -> Self {
        Self {
            route,
            img,
            title: title.to_string(),
            desc: desc.to_string(),
        }
    }
}

#[component]
pub fn Card(props: CardProps) -> Element {
    rsx! {
        Link {
            to: props.route,
            class: "bg-neutral-100 p-2 w-40 h-40 rounded-lg shadow-lg sm:w-44 sm:h-44 md:h-48 md:w-48 lg:w-52 lg:h-52 hover:bg-neutral-200 hover:text-red-500 focus:bg-neutral-200 focus:text-red-500",
            {}
            img {
                src: props.img,
                alt: "merge image",
                class: "w-8 h-8 sm:w-10 sm:h-10 md:w-12 md:h-12 lg:w-14 lg:h-14",
            }
            h2 { class: "my-2 text-base font-medium md:text-2xl", {props.title} }
            p { class: "text-sm md:text-base", {props.desc} }
        }
    }
}
