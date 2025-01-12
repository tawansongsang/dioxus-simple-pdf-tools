use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct DisplayPdf {
    is_display: Memo<bool>,
    object_url: Signal<String>,
}

#[component]
pub fn DisplayPdf(props: DisplayPdf) -> Element {
    let display_iframe = use_memo(move || {
        if *props.is_display.read() {
            "xl:block".to_string()
        } else {
            "".to_string()
        }
    });
    rsx! {
        div { class: "w-1/2 h-auto border-2 border-neutral-500 hidden {display_iframe}",
            if *props.is_display.read() {
                iframe {
                    class: "block",
                    src: props.object_url,
                    name: "iframe_pdf",
                    width: "100%",
                    height: "100%",
                    title: "PDF VIEWER",
                }
            }
        }
    }
}
