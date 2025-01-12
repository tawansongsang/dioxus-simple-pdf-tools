use dioxus::prelude::*;

use crate::{pdf::SplitDocumnet, utils::convert_vec_u8_to_pdf_blob};

use super::input_file::FileUploaded;

#[derive(PartialEq, Clone, Props)]
pub struct SplitInputPageNumberStr {
    #[props(into)]
    name: String,
    #[props(into)]
    example: String,
    is_fiexed_page: bool,
    signal_page_numbers_str: Signal<String>,
    error_message: Signal<String>,
    file_uploaded: Signal<Option<FileUploaded>>,
    files_uploaded: Signal<Vec<FileUploaded>>,
    object_url: Signal<String>,
}

const SPLIT_ACTION: Asset = asset!("/assets/imgs/scissor-2-svgrepo-com.svg");
const PREVIEW: Asset = asset!("/assets/imgs/preview-svgrepo-com.svg");

#[component]
pub fn SplitInputPageNumberStr(mut props: SplitInputPageNumberStr) -> Element {
    let split_action_handler = move |_evt: Event<MouseData>| {
        props.error_message.set(String::new());
        let check_split_page_numbers_str = if props.is_fiexed_page {
            SplitDocumnet::is_valid_string_split_fixed_pages(
                props.signal_page_numbers_str.read().as_str(),
            )
        } else {
            SplitDocumnet::is_valid_string_split_pages(
                props.signal_page_numbers_str.read().as_str(),
            )
        };
        match check_split_page_numbers_str {
            Ok(check_split_page_numbers_str) => {
                if check_split_page_numbers_str {
                    if let Some(file) = props.file_uploaded.read().clone() {
                        let split_file_name = file.get_filename();
                        let buffer = file.file_buffer;
                        let split_pdfs_result = if props.is_fiexed_page {
                            let page_number_u32: u32 = props
                                .signal_page_numbers_str
                                .read()
                                .as_str()
                                .parse()
                                .unwrap_or(1);
                            SplitDocumnet::split_pdf_from_mem_fixed_page(&buffer, page_number_u32)
                        } else {
                            SplitDocumnet::split_pdf_from_mem(
                                &buffer,
                                props.signal_page_numbers_str.read().as_str(),
                            )
                        };

                        match split_pdfs_result {
                            Ok(mut docs) => {
                                props
                                    .files_uploaded
                                    .read()
                                    .iter()
                                    .for_each(|file_uploaded| {
                                        file_uploaded.revoke_object_url();
                                    });
                                props.files_uploaded.set(Vec::new());
                                docs.iter_mut().for_each(|(doc, page)| {
                                    let mut buffer: Vec<u8> = Vec::new();
                                    let write_to_buffer = doc.save_to(&mut buffer);
                                    match write_to_buffer {
                                        Ok(_) => {
                                            let file_data_blob =
                                                convert_vec_u8_to_pdf_blob(&buffer);
                                            match file_data_blob {
                                                Ok(blob) => {
                                                    let file_uploaded = FileUploaded::new(
                                                        format!("{split_file_name}-{page}.pdf"),
                                                        blob,
                                                        buffer,
                                                    );
                                                    match file_uploaded {
                                                        Ok(file) => {
                                                            props.files_uploaded.write().push(file);
                                                        }
                                                        Err(e) => {
                                                            props.error_message.set(format!(
                                                                "Error (split, 88): {e}"
                                                            ));
                                                        }
                                                    }
                                                }
                                                Err(e) => {
                                                    props
                                                        .error_message
                                                        .set(format!("Error (split, 96): {e}"));
                                                }
                                            }
                                        }
                                        Err(std_io_e) => {
                                            props
                                                .error_message
                                                .set(format!("Error (split, 103): {std_io_e}"));
                                        }
                                    }
                                });
                            }
                            Err(split_pdfs_e) => match split_pdfs_e {
                                crate::pdf::Error::Lopdf(error) => props
                                    .error_message
                                    .set(format!("Error::Lopdf (split, 111): {error}")),
                                _ => props
                                    .error_message
                                    .set(format!("Error (split, 114): {split_pdfs_e}")),
                            },
                        }
                    };
                } else {
                    props
                        .error_message
                        .set("invalid split number pattern".to_string())
                }
            }
            Err(e) => props.error_message.set(format!("Error (split, 124): {e}")),
        };
    };

    rsx! {
        div { class: "flex w-4/5 max-w-screen-md items-center justify-center p-1",
            label { class: "mr-2", r#for: "split_str", "{props.name}" }
            input {
                id: "split_str",
                class: "text-base rounded-md border-2 w-60 sm:w-80 border-neutral-300 p-2",
                r#type: "text",
                placeholder: props.example,
                value: props.signal_page_numbers_str,
                oninput: move |evt: Event<FormData>| { props.signal_page_numbers_str.set(evt.value()) },
            }

            img {
                src: SPLIT_ACTION,
                alt: "split_action",
                class: "w-10 h-10 mx-2 cursor-pointer",
                onclick: split_action_handler,
            }
            a {
                title: "Preview PDF",
                class: "hidden xl:block",
                href: props.object_url,
                target: "iframe_pdf",
                img { src: PREVIEW, alt: "preview_pdf", class: "w-10 h-10" }
            }
        }
    }
}
