use chrono::Local;
use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    let start_year = "2025";
    let end_year = Local::now().format("%Y");

    rsx! {
        footer { id: "footer",
            hr { class: "mx-auto my-3 h-px w-[95vw] border-0 bg-neutral-700" }
            p { class: "text-center text-xs md:text-sm",
                span {
                    "Copyright © "
                    time { "{start_year}-{end_year} " }
                }
                span { "Tawansongsang Karnkawinpong" }
            }
        }
    }
}
