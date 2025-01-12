use dioxus::prelude::*;

use crate::components::ItemAction;

use super::input_file::FileUploaded;

const DOWNLOAD_IMG: Asset = asset!("/assets/imgs/download-svgrepo-com.svg",);
const TRASH_IMG: Asset = asset!("/assets/imgs/trash-svgrepo-com.svg",);
const UP_IMG: Asset = asset!("/assets/imgs/up-svgrepo-com.svg",);
const DOWN_IMG: Asset = asset!("/assets/imgs/down-svgrepo-com.svg",);

#[derive(PartialEq, Clone, Props)]
pub struct ListPdfProps {
    filename: String,
    url: String,
    files_uploaded: Signal<Vec<FileUploaded>>,
    idx: usize,
}

#[component]
pub fn ListPdf(props: ListPdfProps) -> Element {
    rsx! {
        li { class: "flex flex-row items-center justify-between my-1",
            a {
                class: "w-full cursor-pointer truncate rounded-md p-1 text-xs hover:bg-neutral-200 focus:bg-neutral-200 sm:text-sm md:text-base",
                title: props.filename,
                href: props.url,
                target: "iframe_pdf",
                "{props.filename}"
            }

            ItemAction {
                object_url: props.url.clone(),
                filename: props.filename.clone(),
                download_img: DOWNLOAD_IMG,
                trash_img: TRASH_IMG,
                up_img: UP_IMG,
                down_img: DOWN_IMG,
                files_uploaded: props.files_uploaded,
                idx: props.idx,
            }
        }
    }
}
