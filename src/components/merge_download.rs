use dioxus::prelude::*;
use web_sys::Url;

use crate::{error::Error, pdf::MergeDocument, utils::convert_vec_u8_to_pdf_blob};

use super::input_file::FileUploaded;

#[derive(PartialEq, Clone, Props)]
pub struct MergeDownloadProps {
    merge_file_name: Signal<String>,
    merge_file_object_url: Signal<String>,
    files_uploaded: Signal<Vec<FileUploaded>>,
    error_message: Signal<String>,
}

#[component]
pub fn MergeDownload(mut props: MergeDownloadProps) -> Element {
    let merge_file_handler = move |_evt: Event<MouseData>| {
        let document = MergeDocument::merge_pdf_from_mem(
            props
                .files_uploaded
                .read()
                .iter()
                .filter_map(|file_uploaded| {
                    let FileUploaded { file_buffer, .. } = file_uploaded;
                    Some(file_buffer.as_slice())
                })
                .collect(),
        );
        match document {
            Ok(mut doc) => {
                if !props.merge_file_object_url.read().is_empty() {
                    let _revoke_url = Url::revoke_object_url(&props.merge_file_object_url.read());
                }
                let mut buffer: Vec<u8> = Vec::new();
                let write_to_buffer = doc.save_to(&mut buffer);
                match write_to_buffer {
                    Ok(_) => {
                        let file_data_blob = convert_vec_u8_to_pdf_blob(&buffer);
                        match file_data_blob {
                            Ok(file_data_blob) => {
                                let result_url = Url::create_object_url_with_blob(&file_data_blob)
                                    .map_err(|js_value| {
                                        Error::JsValue(format!("Error (merge, 44): {js_value:?}"))
                                    });
                                match result_url {
                                    Ok(url) => {
                                        props.merge_file_object_url.set(url);
                                        props.merge_file_name.set("merge.pdf".to_string());
                                    }
                                    Err(e) => {
                                        props.error_message.set(format!("Error (merge, 52): {e}"));
                                    }
                                }
                            }
                            Err(e) => {
                                props.error_message.set(format!("Error (merge, 57): {e}"));
                            }
                        }
                    }
                    Err(std_io_e) => {
                        props
                            .error_message
                            .set(format!("Error (merge, 64): {std_io_e}"));
                    }
                };
            }
            Err(pdf_merge_e) => match pdf_merge_e {
                crate::pdf::Error::Lopdf(error) => props
                    .error_message
                    .set(format!("Error::Lopdf (merge, 71): {error}")),
                _ => props
                    .error_message
                    .set(format!("Error (merge, 74): {pdf_merge_e}")),
            },
        }
    };
    rsx! {
        div { class: "mt-4 flex w-auto flex-col items-center rounded-md p-4",
            button {
                id: "merge",
                class: "w-1/2 rounded-md bg-neutral-200 p-2 text-sm hover:bg-neutral-400 focus:bg-neutral-400 sm:text-base",
                onclick: merge_file_handler,
                "Merge PDF"
            }
            div { class: "mt-3 flex flex-col w-full max-w-screen-sm",
                if props.merge_file_name.read().is_empty() {
                    p { class: "text-center text-sm text-red-400 sm:text-base", "No files to merge" }
                } else {
                    label { class: "text-sm pl-2", r#for: "filename", "Filename" }
                    div { class: "flex justify-between gap-2",
                        input {
                            id: "merge_filename",
                            class: "text-base rounded-md border-2 w-full border-neutral-100 p-2",
                            r#type: "text",
                            value: props.merge_file_name,
                            oninput: move |evt: Event<FormData>| props.merge_file_name.set(evt.value()),
                        }
                        a {
                            class: "rounded-md p-2 bg-sky-200 hover:bg-sky-400 focus:bg-sky-400",
                            href: props.merge_file_object_url,
                            download: props.merge_file_name,
                            "Download"
                        }
                    }
                }
            }
        }
    }
}
