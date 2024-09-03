#![warn(clippy::pedantic)]
pub mod blend_ops;
pub mod dynamic_blend;
// pub mod helpers;
mod enums;
mod error;
pub use error::Error;
pub mod alpha_ops;
pub mod pixelops;
mod tests;
pub use alpha_ops::BufferGetAlpha;
pub use alpha_ops::BufferSetAlpha;
pub use blend_ops::BufferBlend;
pub use dynamic_blend::DynamicChops;
