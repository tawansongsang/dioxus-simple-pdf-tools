use crate::{
    components::{Card, CardProps, Hero},
    routes::Route,
};
use dioxus::prelude::*;

const MERGE_PDF_IMAGE: Asset = asset!("/assets/imgs/merge-cells-svgrepo-com.svg");
const SPLIT_PDF_IMAGE: Asset = asset!("/assets/imgs/split-svgrepo-com.svg");

#[component]
pub fn Home() -> Element {
    let card_props = vec![
        CardProps::new(
            Route::Merge {},
            MERGE_PDF_IMAGE,
            "Merge PDF",
            "Combine PDFs into one",
        ),
        CardProps::new(
            Route::Split {},
            SPLIT_PDF_IMAGE,
            "Split PDF",
            "Separate PDFs into multiple files",
        ),
    ];
    rsx! {
        Hero { title: "DIOXUS SIMPLE PDF TOOLS" }
        main {
            id: "main-content",
            class: "mx-3 flex h-[50vh] flex-row flex-wrap justify-center gap-2 sm:mx-6 sm:gap-3 md:gap-4",
            for card_prop in card_props {
                Card {
                    route: card_prop.route,
                    img: card_prop.img,
                    title: card_prop.title,
                    desc: card_prop.desc,
                }
            }
        }
    }
}
