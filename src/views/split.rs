use dioxus::prelude::*;
use dioxus_elements::HasFileData;

use crate::{
    components::{
        input_file::{split_input_file::SplitInputFile, FileUploaded},
        DropArea, Hero, InputFile, ResultPdf, SplitAction,
    },
    utils::convert_vec_u8_to_pdf_blob,
};

#[component]
pub fn Split() -> Element {
    let mut file_uploaded: Signal<Option<FileUploaded>> = use_signal(|| None);
    let mut files_uploaded: Signal<Vec<FileUploaded>> = use_signal(Vec::new);
    let mut error_message = use_signal(String::new);
    let mut object_url = use_signal(String::new);
    let is_display = use_memo(move || file_uploaded().is_some());
    let mut is_drag = use_signal(|| false);

    let drag_upload_file_handler = move |evt: Event<DragData>| {
        evt.prevent_default();
        is_drag.set(false);
        async move {
            if let Some(file_engine) = evt.files() {
                for old_file in files_uploaded.read().clone() {
                    old_file.revoke_object_url();
                }
                files_uploaded.write().clear();
                let files = file_engine.files();
                for filename in files {
                    if let Some(file_buffer) = file_engine.read_file(&filename).await {
                        let file_data_blob = convert_vec_u8_to_pdf_blob(&file_buffer);
                        match file_data_blob {
                            Ok(file_data_blob) => {
                                let result_file_uploaded =
                                    FileUploaded::new(filename, file_data_blob, file_buffer);
                                match result_file_uploaded {
                                    Ok(file) => {
                                        if let Some(current_file_uploaded) =
                                            file_uploaded.read().clone()
                                        {
                                            current_file_uploaded.revoke_object_url();
                                        }
                                        object_url.set(file.get_url());
                                        file_uploaded.set(Some(file));
                                    }
                                    Err(e) => error_message.set(format!("Error (86): {e}")),
                                }
                            }
                            Err(e) => error_message.set(format!("Error (90): {e}")),
                        }
                    }
                }
            }
        }
    };

    rsx! {
        Hero { title: "Split PDF" }
        main {
            id: "split",
            class: "flex h-screen w-full flex-col items-center",
            ondragenter: move |evt| {
                evt.prevent_default();
                is_drag.set(true);
            },
            ondragover: move |evt| {
                evt.prevent_default();
                is_drag.set(true);
            },
            ondragleave: move |evt| {
                evt.prevent_default();
                is_drag.set(false);
            },
            ondrop: drag_upload_file_handler,
            InputFile {
                name: "Selete PDF File for split",
                file_input_element: rsx! {
                    SplitInputFile {
                        file_uploaded,
                        files_uploaded,
                        object_url,
                        error_message,
                    }
                },
            }
            if !error_message().is_empty() {
                p { class: "text-center text-red-500", {error_message} }
            }
            // SplitAction { is_display }
            if is_display() {
                SplitAction {
                    error_message,
                    file_uploaded,
                    files_uploaded,
                    object_url,
                }
            }
            ResultPdf {
                is_display,
                object_url,
                files_uploaded,
                error_message,
            }
            if is_drag() {
                DropArea { is_drag }
            }
        }
    }
}
