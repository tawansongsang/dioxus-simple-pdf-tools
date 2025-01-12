pub mod merge_input_file;
pub mod split_input_file;

use dioxus::prelude::*;
use web_sys::{Blob, Url};

use crate::error::Error;

#[derive(PartialEq, Props, Clone)]
pub struct InputFileProps {
    name: String,
    file_input_element: Element,
}

#[component]
pub fn InputFile(props: InputFileProps) -> Element {
    rsx! {
        section { class: "flex h-[12.5%] w-4/5 max-w-screen-md items-center justify-center rounded-md border-2 border-dashed border-neutral-600 p-4 sm:h-1/6",
            {props.file_input_element}
            label {
                class: "h-fit cursor-pointer rounded-md bg-neutral-200 p-2 text-sm hover:bg-neutral-400 focus:bg-neutral-400 sm:p-3 sm:text-base",
                r#for: "files",
                "{props.name}"
            }
        }
    }
}

// region:    --- Enum and Struct For File Uploaded and Object URL
#[derive(Clone)]
pub struct FileUploaded {
    pub filename: String,
    pub url: String,
    pub file_buffer: Vec<u8>,
}

impl FileUploaded {
    pub fn new(filename: String, blob: Blob, file_buffer: Vec<u8>) -> Result<Self, Error> {
        let url = Url::create_object_url_with_blob(&blob)
            .map_err(|_| Error::JsValue("Could not create object URL".to_string()))?;
        Ok(Self {
            filename,
            url,
            file_buffer,
        })
    }

    pub fn get_url(&self) -> String {
        self.url.clone()
    }

    pub fn revoke_object_url(&self) {
        let _revoke_oveject_url = Url::revoke_object_url(&self.url);
    }

    pub fn get_filename(&self) -> String {
        self.filename[..self.filename.len() - 4].to_string()
    }
}

// endregion: --- Enum and Struct For File Uploaded and Object URL
