// region:    --- Error
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    // Lopdf Error from lopdf crate
    Lopdf(lopdf::Error),

    // Error for merge::Merge
    PageObjectNotFound,
    CatalogObjectNotFound,

    // Error For split::Split
    SplitPagesStrIsEmpty,
    CannotCreateRegex,
    InValidPageNumbers,
    PageNumberOverFlow,
    FiexedPageNumberOverFlow,
}
// endregion: --- Error

// region:    --- Error Boilerplate
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
