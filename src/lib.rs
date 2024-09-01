#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
mod dynamic_blend;
pub mod blend_ops;
mod enums;
mod error;
mod pixelops;
mod tests;
