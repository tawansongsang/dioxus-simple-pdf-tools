use dioxus::prelude::*;

use crate::components::{DisplayPdf, ListPdf};

use super::input_file::FileUploaded;

#[derive(PartialEq, Clone, Props)]
pub struct ResultPdfProps {
    is_display: Memo<bool>,
    object_url: Signal<String>,
    files_uploaded: Signal<Vec<FileUploaded>>,
    error_message: Signal<String>,
    download_element: Option<Element>,
}

#[component]
pub fn ResultPdf(props: ResultPdfProps) -> Element {
    rsx! {
        section { class: "mt-4 flex h-full w-4/5 max-w-screen-md flex-row justify-between overflow-auto rounded-md xl:w-[95%] xl:max-w-full",
            div { class: "w-full h-4/5 xl:w-1/2",
                if *props.is_display.read() {
                    ul { class: "flex h-auto max-h-full overflow-auto flex-col rounded-md border-2 border-neutral-300 xl:mr-2",
                        for (idx , file_uploaded) in props.files_uploaded.read().iter().enumerate() {
                            {
                                let FileUploaded { filename, url, .. } = file_uploaded;
                                rsx! {
                                    ListPdf {
                                        filename,
                                        url,
                                        files_uploaded: props.files_uploaded,
                                        idx,
                                    }
                                }
                            }
                        }
                    }
                    {props.download_element}
                }
            }
            DisplayPdf { is_display: props.is_display, object_url: props.object_url }
        }
    }
}
