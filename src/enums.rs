use image::{flat::SampleLayout, ColorType};

use crate::error::Error;

pub(crate) enum ColorStructure {
    L,
    La,
    Rgb,
    Rgba,
}
impl TryFrom<SampleLayout> for ColorStructure {
    fn try_from(colour_type: SampleLayout) -> Result<Self, Error> {
        match colour_type.channels {
            1 => Ok(ColorStructure::L),
            2 => Ok(ColorStructure::La),
            3 => Ok(ColorStructure::Rgb),
            4 => Ok(ColorStructure::Rgba),
            _ => Err(Error::UnsupportedType),
        }
    }

    type Error = Error;
}
impl ColorStructure {
    pub(crate) fn alpha(&self) -> bool {
        match self {
            ColorStructure::La | ColorStructure::Rgba => true,
            _ => false,
        }
    }
    pub(crate) fn rgb (&self) -> bool {
        match self {
            ColorStructure::L | ColorStructure::La => false,
            ColorStructure::Rgb | ColorStructure::Rgba => true,
        }
    }
    pub(crate) fn alpha_channel(&self) -> Option<usize> {
        match self {
            ColorStructure::La => Some(1),
            ColorStructure::Rgba => Some(3),
            _ => None,
        }
    }
}
pub(crate) trait ColorString {
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
