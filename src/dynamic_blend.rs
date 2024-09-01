use std::ops::DerefMut;

use image::{ColorType, DynamicImage, ImageBuffer, Pixel};

use crate::{blend_ops::Blend, error::Error};

pub trait DynamicBlend {
    fn blend (
        &mut self,
        other: &Self,
        op: fn(f64, f64) -> f64,
        apply_to_color: bool,
        apply_to_alpha: bool,
    ) -> Result<(), Error>;
}
impl DynamicBlend for DynamicImage {
    fn blend (
        &mut self,
        other: &Self,
        op: fn(f64, f64) -> f64,
        apply_to_color: bool,
        apply_to_alpha: bool,
    ) -> Result<(), Error> {
        match self.color() {
            ColorType::L8 => step_a(self.as_mut_luma8().unwrap(), other, op, apply_to_color, apply_to_alpha),
            ColorType::La8 => step_a(self.as_mut_luma_alpha8().unwrap(), other, op, apply_to_color, apply_to_alpha),
            ColorType::Rgb8 => step_a(self.as_mut_rgb8().unwrap(), other, op, apply_to_color, apply_to_alpha),
            ColorType::Rgba8 => step_a(self.as_mut_rgba8().unwrap(), other, op, apply_to_color, apply_to_alpha),
            ColorType::L16 => step_a(self.as_mut_luma16().unwrap(), other, op, apply_to_color, apply_to_alpha),
            ColorType::La16 => step_a(self.as_mut_luma_alpha16().unwrap(), other, op, apply_to_color, apply_to_alpha),
            ColorType::Rgb16 => step_a(self.as_mut_rgb16().unwrap(), other, op, apply_to_color, apply_to_alpha),
            ColorType::Rgba16 => step_a(self.as_mut_rgba16().unwrap(), other, op, apply_to_color, apply_to_alpha),
            ColorType::Rgb32F => step_a(self.as_mut_rgb32f().unwrap(), other, op, apply_to_color, apply_to_alpha),
            ColorType::Rgba32F => step_a(self.as_mut_rgba32f().unwrap(), other, op, apply_to_color, apply_to_alpha),
            _ => Err(Error::UnsupportedType),

        }
    }
}
fn step_a<Pmut, ContainerMut>(subject: &mut ImageBuffer<Pmut, ContainerMut>, other: &DynamicImage, op: fn(f64, f64) -> f64, apply_to_color: bool, apply_to_alpha: bool) -> Result<(), Error>
where 
    Pmut: Pixel,
    ContainerMut: DerefMut<Target = [Pmut::Subpixel]>
    + DerefMut<Target = [Pmut::Subpixel]>
    + AsMut<[<Pmut as Pixel>::Subpixel]>,
{
    match other.color() {
        ColorType::L8 => subject.blend(other.as_luma8().unwrap(), op, apply_to_color, apply_to_alpha),
        ColorType::La8 => subject.blend(other.as_luma_alpha8().unwrap(), op, apply_to_color, apply_to_alpha),
        ColorType::Rgb8 => subject.blend(other.as_rgb8().unwrap(), op, apply_to_color, apply_to_alpha),
        ColorType::Rgba8 => subject.blend(other.as_rgba8().unwrap(), op, apply_to_color, apply_to_alpha),
        ColorType::L16 => subject.blend(other.as_luma16().unwrap(), op, apply_to_color, apply_to_alpha),
        ColorType::La16 => subject.blend(other.as_luma_alpha16().unwrap(), op, apply_to_color, apply_to_alpha),
        ColorType::Rgb16 => subject.blend(other.as_rgb16().unwrap(), op, apply_to_color, apply_to_alpha),
        ColorType::Rgba16 => subject.blend(other.as_rgba16().unwrap(), op, apply_to_color, apply_to_alpha),
        ColorType::Rgb32F => subject.blend(other.as_rgb32f().unwrap(), op, apply_to_color, apply_to_alpha),
        ColorType::Rgba32F => subject.blend(other.as_rgba32f().unwrap(), op, apply_to_color, apply_to_alpha),
        _ => Err(Error::UnsupportedType),
    }
}
