#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Image dimensions do not match")]
    DimensionMismatch,

    #[error("Attempted to blend images with an unsupported color type")]
    UnsupportedType,

    #[error("Image 'a' of type {0} cannot accept blends from image 'b' of type {1}")]
    UnsupportedBlend(&'static str, &'static str),

    #[error("Cannot access alpha channel as image does not have an alpha channel")]
    NoAlphaChannel,
}
