pub type Result<T> = std::result::Result<T, Error>;

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Error {
    // JsValue from web_sys crate
    JsValue(String),
}

// region:    --- Error Boilerplate
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
