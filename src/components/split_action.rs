use dioxus::prelude::*;

use crate::components::SplitInputPageNumberStr;

use super::input_file::FileUploaded;

#[derive(PartialEq, Clone, Props)]
pub struct SplitActionProps {
    error_message: Signal<String>,
    file_uploaded: Signal<Option<FileUploaded>>,
    files_uploaded: Signal<Vec<FileUploaded>>,
    object_url: Signal<String>,
}

#[component]
pub fn SplitAction(props: SplitActionProps) -> Element {
    let split_page_numbers_str = use_signal(String::new);
    let split_fixed_page_numbers_str = use_signal(String::new);

    rsx! {
        section { class: "mt-2 flex flex-col items-center w-full",
            SplitInputPageNumberStr {
                name: "Select pages: ",
                example: "Example: 1, 2-3, 5",
                signal_page_numbers_str: split_page_numbers_str,
                is_fiexed_page: false,
                error_message: props.error_message,
                file_uploaded: props.file_uploaded,
                files_uploaded: props.files_uploaded,
                object_url: props.object_url,
            }
            SplitInputPageNumberStr {
                name: "Fixed ranges: ",
                example: "Example: 2",
                signal_page_numbers_str: split_fixed_page_numbers_str,
                is_fiexed_page: true,
                error_message: props.error_message,
                file_uploaded: props.file_uploaded,
                files_uploaded: props.files_uploaded,
                object_url: props.object_url,
            }
        }
    }
}
