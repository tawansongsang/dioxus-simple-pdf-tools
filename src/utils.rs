use web_sys::js_sys::{Array, Uint8Array};
use web_sys::{Blob, BlobPropertyBag};

use crate::error::{Error, Result};

pub fn convert_vec_u8_to_pdf_blob(buffer: &Vec<u8>) -> Result<Blob> {
    let uint8arr_view = unsafe { Uint8Array::view(buffer) };
    let uint8arr = Uint8Array::new(&uint8arr_view);
    let array = Array::new();
    array.push(&uint8arr.buffer());
    let blob_options = BlobPropertyBag::new();
    blob_options.set_type("application/pdf");
    let blob =
        Blob::new_with_u8_array_sequence_and_options(&array, &blob_options).map_err(|e| {
            if e.is_null() {
                return Error::JsValue("Get Null From Pdf File".to_string());
            } else if e.is_undefined() {
                return Error::JsValue("Get Undefined From Pdf File".to_string());
            } else if e.is_falsy() {
                return Error::JsValue("Get False From Pdf File".to_string());
            } else {
                return Error::JsValue("Error Converting PDF File to Blob".to_string());
            }
        })?;

    Ok(blob)
}
