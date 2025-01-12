use dioxus::prelude::*;

use super::input_file::FileUploaded;

#[derive(PartialEq, Clone, Props)]
pub struct ItemActionProps {
    object_url: Option<String>,
    filename: Option<String>,
    download_img: Option<Asset>,
    trash_img: Option<Asset>,
    up_img: Option<Asset>,
    down_img: Option<Asset>,
    files_uploaded: Signal<Vec<FileUploaded>>,
    idx: usize,
}

#[component]
pub fn ItemAction(mut props: ItemActionProps) -> Element {
    rsx! {
        div { class: "flex w-36 flex-row justify-end sm:w-40 md:w-48",
            if let Some(up_img) = props.up_img {
                img {
                    class: "w-6 cursor-pointer rounded-md p-1 hover:bg-neutral-200 focus:bg-neutral-200 sm:w-7 md:w-8",
                    src: up_img,
                    alt: "Up",
                    onclick: move |_evt| {
                        if props.idx > 0 {
                            props.files_uploaded.write().swap(props.idx, props.idx - 1);
                        }
                    },
                }
            }
            if let Some(down_img) = props.down_img {
                img {
                    class: "w-6 cursor-pointer rounded-md p-1 hover:bg-neutral-200 focus:bg-neutral-200 sm:w-7 md:w-8",
                    src: down_img,
                    alt: "Down",
                    onclick: move |_evt| {
                        if props.idx < props.files_uploaded.read().len() - 1 {
                            props.files_uploaded.write().swap(props.idx, props.idx + 1);
                        }
                    },
                }
            }
            if let Some(trash_img) = props.trash_img {
                img {
                    class: "w-6 cursor-pointer rounded-md p-1 hover:bg-neutral-200 focus:bg-neutral-200 sm:w-7 md:w-8",
                    src: trash_img,
                    alt: "Delete",
                    onclick: move |_evt| {
                        props.files_uploaded.read()[props.idx].revoke_object_url();
                        props.files_uploaded.write().remove(props.idx);
                    },
                }
            }
            if let (Some(object_url), Some(filename), Some(download_img)) = (
                props.object_url,
                props.filename,
                props.download_img,
            )
            {
                a {
                    class: "w-6 cursor-pointer rounded-md p-1 hover:bg-neutral-200 focus:bg-neutral-200 sm:w-7 md:w-8",
                    href: "{object_url}",
                    download: "{filename}",
                    img { src: download_img, alt: "Download" }
                }
            }
        }
    }
}
