use blend_ops::ColorStructure;
use image::{ColorType, DynamicImage};

mod pixelops;
mod error;
mod blend_ops;
mod tests;

trait ColorString {
    fn color_str(&self) -> &'static str;
}

impl ColorString for ColorType {
    fn color_str(&self) -> &'static str
    {
        match self {
            ColorType::L8 => "L8",
            ColorType::La8 => "La8",
            ColorType::Rgb8 => "Rgb8",
            ColorType::Rgba8 => "Rgba8",
            ColorType::L16 => "L16",
            ColorType::La16 => "La16",
            ColorType::Rgb16 => "Rgb16",
            ColorType::Rgba16 => "Rgba16",
            ColorType::Rgb32F => "Rgb32F",
            ColorType::Rgba32F => "Rgba32F",
            _ => "Unknown"
        }
    }
}
impl ColorString for ColorStructure {
    fn color_str(&self) -> &'static str
    {
        match self {
            ColorStructure::L => "L",
            ColorStructure::La => "La",
            ColorStructure::Rgb => "Rgb",
            ColorStructure::Rgba => "Rgba",
        }
    }
}
