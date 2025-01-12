use dioxus::prelude::*;
use dioxus_elements::HasFileData;

use crate::{
    components::{
        input_file::FileUploaded, DropArea, Hero, InputFile, MergeDownload, MergeInputFile,
        ResultPdf,
    },
    utils::convert_vec_u8_to_pdf_blob,
};

#[component]
pub fn Merge() -> Element {
    let mut files_uploaded: Signal<Vec<FileUploaded>> = use_signal(Vec::new);
    let mut merge_file_name = use_signal(|| "".to_string());
    let merge_file_object_url = use_signal(|| "".to_string());
    let mut error_message = use_signal(|| "".to_string());
    let is_display = use_memo(move || !files_uploaded.read().is_empty());
    let mut is_drag = use_signal(|| false);

    let drag_upload_file_handler = move |evt: Event<DragData>| {
        evt.prevent_default();
        is_drag.set(false);
        async move {
            if let Some(file_engine) = evt.files() {
                let files = file_engine.files();
                for filename in files {
                    if let Some(file_buffer) = file_engine.read_file(&filename).await {
                        let file_data_blob = convert_vec_u8_to_pdf_blob(&file_buffer);
                        match file_data_blob {
                            Ok(file_data_blob) => {
                                let result_file_uploaded =
                                    FileUploaded::new(filename, file_data_blob, file_buffer);
                                match result_file_uploaded {
                                    Ok(file_uploaded) => {
                                        files_uploaded.write().push(file_uploaded);
                                        merge_file_name.write().clear();
                                    }
                                    Err(e) => error_message
                                        .set(format!("Error (merge_input_file, 34): {e}")),
                                }
                            }
                            Err(e) => {
                                error_message.set(format!("Error (merge_input_file, 39): {e}"))
                            }
                        }
                    }
                }
            }
        }
    };

    rsx! {
        Hero { title: "Merge PDF" }
        main {
            id: "merge",
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
                name: "Select PDF Files to Merge",
                file_input_element: rsx! {
                    MergeInputFile { files_uploaded, merge_file_name, error_message }
                },
            }
            if !error_message().is_empty() {
                p { class: "text-center text-red-500", {error_message} }
            }

            ResultPdf {
                is_display,
                object_url: merge_file_object_url,
                files_uploaded,
                error_message,
                download_element: rsx! {
                    MergeDownload {
                        merge_file_name,
                        merge_file_object_url,
                        files_uploaded,
                        error_message,
                    }
                },
            }
            if is_drag() {
                DropArea { is_drag }
            }

        }
    }
}
