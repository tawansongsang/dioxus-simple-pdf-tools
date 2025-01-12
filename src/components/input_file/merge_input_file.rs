use dioxus::prelude::*;

use crate::{components::input_file::FileUploaded, utils::convert_vec_u8_to_pdf_blob};

#[derive(PartialEq, Clone, Props)]
pub struct MergeInputFileProps {
    files_uploaded: Signal<Vec<FileUploaded>>,
    merge_file_name: Signal<String>,
    error_message: Signal<String>,
}

#[component]
pub fn MergeInputFile(mut props: MergeInputFileProps) -> Element {
    let upload_file_handler = move |evt: Event<FormData>| async move {
        if let Some(file_engine) = &evt.files() {
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
                                    props.files_uploaded.write().push(file_uploaded);
                                    props.merge_file_name.write().clear();
                                }
                                Err(e) => props
                                    .error_message
                                    .set(format!("Error (merge_input_file, 34): {e}")),
                            }
                        }
                        Err(e) => props
                            .error_message
                            .set(format!("Error (merge_input_file, 39): {e}")),
                    }
                }
            }
        }
    };

    rsx! {
        input {
            id: "files",
            class: "hidden",
            // tell the input to pick a file
            r#type: "file",
            // list the accepted extensions
            accept: ".pdf",
            // pick multiple files
            multiple: true,
            onchange: upload_file_handler,
        }
    }
}
