use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct MergeDropProps {
    is_drag: Signal<bool>,
}

#[component]
pub fn DropArea(props: MergeDropProps) -> Element {
    let css_drag = use_memo(move || {
        if *props.is_drag.read() {
            "bg-slate-950 opacity-40"
        } else {
            ""
        }
    });

    rsx! {
        div { class: "{css_drag} absolute z-[-1] left-0 top-14 flex flex-col h-screen w-screen justify-center",
            if *props.is_drag.read() {
                p { class: "text-center text-neutral-100 text-xl sm:text-2xl md:text-3xl",
                    "Drop file here"
                }
            }
        }
    }
}
