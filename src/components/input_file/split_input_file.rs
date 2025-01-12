use dioxus::prelude::*;

use crate::utils::convert_vec_u8_to_pdf_blob;

use super::FileUploaded;

#[derive(PartialEq, Clone, Props)]
pub struct SplitInputFileProps {
    file_uploaded: Signal<Option<FileUploaded>>,
    files_uploaded: Signal<Vec<FileUploaded>>,
    object_url: Signal<String>,
    error_message: Signal<String>,
}

#[component]
pub fn SplitInputFile(mut props: SplitInputFileProps) -> Element {
    let upload_file_handler = move |evt: Event<FormData>| async move {
        if let Some(file_engine) = &evt.files() {
            for old_file in props.files_uploaded.read().clone() {
                old_file.revoke_object_url();
            }
            props.files_uploaded.write().clear();
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
                                    if let Some(current_file_uploaded) =
                                        props.file_uploaded.read().clone()
                                    {
                                        current_file_uploaded.revoke_object_url();
                                    }
                                    props.object_url.set(file_uploaded.get_url());
                                    props.file_uploaded.set(Some(file_uploaded));
                                }
                                Err(e) => props.error_message.set(format!("Error (86): {e}")),
                            }
                        }
                        Err(e) => props.error_message.set(format!("Error (90): {e}")),
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
            onchange: upload_file_handler,
        }
    }
}
