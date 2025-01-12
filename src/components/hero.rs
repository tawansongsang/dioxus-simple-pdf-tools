use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct HeroProps {
    title: String,
}

#[component]
pub fn Hero(props: HeroProps) -> Element {
    rsx! {
        section { id: "hero",
            h1 {
                id: "hero-title",
                class: "mb-6 mt-6 text-center text-4xl font-bold md:text-5xl lg:text-6xl",
                "{props.title}"
            }

        }
    }
}
