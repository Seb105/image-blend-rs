use std::{iter::{zip, Product}, ops::{Deref, DerefMut}};

use image::{ColorType, DynamicImage, GenericImageView, ImageBuffer, Luma, Pixel};

use crate::{alpha_ops::{GetAlpha, SetAlpha}, blend_ops::Blend, enums::ColorStructure, error::{self, Error}};

pub trait DynamicBlend {
    fn blend (
        &mut self,
        other: &Self,
        op: fn(f64, f64) -> f64,
        apply_to_color: bool,
        apply_to_alpha: bool,
    ) -> Result<(), Error>;
    // Get alpha as a grayscale image. Output type is same as input type.
    fn get_alpha(
        &self,
    ) -> Result<Self, Error> where Self: std::marker::Sized;
    // Transplant the alpha from image other to self.
    fn transplant_alpha(
        &mut self,
        other: &Self
    ) -> Result<(), Error>;
    // Set alpha from a grayscale image. Note that input type can be any type, but only the first channel will be used.
    // So, if you want to set alpha from a color image, you should convert it to grayscale first.
    fn set_alpha(
        &mut self,
        other: &Self
    ) -> Result<(), Error> where Self: std::marker::Sized;
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
            ColorType::L8 => blend_step_a(self.as_mut_luma8().unwrap(), other, op, apply_to_color, apply_to_alpha),
            ColorType::La8 => blend_step_a(self.as_mut_luma_alpha8().unwrap(), other, op, apply_to_color, apply_to_alpha),
            ColorType::Rgb8 => blend_step_a(self.as_mut_rgb8().unwrap(), other, op, apply_to_color, apply_to_alpha),
            ColorType::Rgba8 => blend_step_a(self.as_mut_rgba8().unwrap(), other, op, apply_to_color, apply_to_alpha),
            ColorType::L16 => blend_step_a(self.as_mut_luma16().unwrap(), other, op, apply_to_color, apply_to_alpha),
            ColorType::La16 => blend_step_a(self.as_mut_luma_alpha16().unwrap(), other, op, apply_to_color, apply_to_alpha),
            ColorType::Rgb16 => blend_step_a(self.as_mut_rgb16().unwrap(), other, op, apply_to_color, apply_to_alpha),
            ColorType::Rgba16 => blend_step_a(self.as_mut_rgba16().unwrap(), other, op, apply_to_color, apply_to_alpha),
            ColorType::Rgb32F => blend_step_a(self.as_mut_rgb32f().unwrap(), other, op, apply_to_color, apply_to_alpha),
            ColorType::Rgba32F => blend_step_a(self.as_mut_rgba32f().unwrap(), other, op, apply_to_color, apply_to_alpha),
            _ => Err(Error::UnsupportedType),

        }
    }
    fn get_alpha(
        &self,
    ) -> Result<DynamicImage, Error> {
        let color = self.color();
        let mut copy = self.clone();
        match color {
            ColorType::L8 => get_alpha_step_a(copy.as_mut_luma8().unwrap()),
            ColorType::La8 => get_alpha_step_a(copy.as_mut_luma_alpha8().unwrap()),
            ColorType::Rgb8 => get_alpha_step_a(copy.as_mut_rgb8().unwrap()),
            ColorType::Rgba8 => get_alpha_step_a(copy.as_mut_rgba8().unwrap()),
            ColorType::L16 => get_alpha_step_a(copy.as_mut_luma16().unwrap()),
            ColorType::La16 => get_alpha_step_a(copy.as_mut_luma_alpha16().unwrap()),
            ColorType::Rgb16 => get_alpha_step_a(copy.as_mut_rgb16().unwrap()),
            ColorType::Rgba16 => get_alpha_step_a(copy.as_mut_rgba16().unwrap()),
            ColorType::Rgb32F => get_alpha_step_a(copy.as_mut_rgb32f().unwrap()),
            ColorType::Rgba32F => get_alpha_step_a(copy.as_mut_rgba32f().unwrap()),
            _ => Err(Error::UnsupportedType),
        }?;
        Ok(copy.grayscale())
    }
    fn transplant_alpha(
            &mut self,
            other: &Self
    ) -> Result<(), Error> {
        match self.color() {
            ColorType::L8 => transplant_alpha_step_a(self.as_mut_luma8().unwrap(), other),
            ColorType::La8 => transplant_alpha_step_a(self.as_mut_luma_alpha8().unwrap(), other),
            ColorType::Rgb8 => transplant_alpha_step_a(self.as_mut_rgb8().unwrap(), other),
            ColorType::Rgba8 => transplant_alpha_step_a(self.as_mut_rgba8().unwrap(), other),
            ColorType::L16 => transplant_alpha_step_a(self.as_mut_luma16().unwrap(), other),
            ColorType::La16 => transplant_alpha_step_a(self.as_mut_luma_alpha16().unwrap(), other),
            ColorType::Rgb16 => transplant_alpha_step_a(self.as_mut_rgb16().unwrap(), other),
            ColorType::Rgba16 => transplant_alpha_step_a(self.as_mut_rgba16().unwrap(), other),
            ColorType::Rgb32F => transplant_alpha_step_a(self.as_mut_rgb32f().unwrap(), other),
            ColorType::Rgba32F => transplant_alpha_step_a(self.as_mut_rgba32f().unwrap(), other),
            _ => Err(Error::UnsupportedType),
        }?;
        Ok(())
    }
    fn set_alpha(
        &mut self,
        other: &Self
    ) -> Result<(), Error> {
        match self.color() {
            ColorType::L8 => set_alpha_step_a(self.as_mut_luma8().unwrap(), other),
            ColorType::La8 => set_alpha_step_a(self.as_mut_luma_alpha8().unwrap(), other),
            ColorType::Rgb8 => set_alpha_step_a(self.as_mut_rgb8().unwrap(), other),
            ColorType::Rgba8 => set_alpha_step_a(self.as_mut_rgba8().unwrap(), other),
            ColorType::L16 => set_alpha_step_a(self.as_mut_luma16().unwrap(), other),
            ColorType::La16 => set_alpha_step_a(self.as_mut_luma_alpha16().unwrap(), other),
            ColorType::Rgb16 => set_alpha_step_a(self.as_mut_rgb16().unwrap(), other),
            ColorType::Rgba16 => set_alpha_step_a(self.as_mut_rgba16().unwrap(), other),
            ColorType::Rgb32F => set_alpha_step_a(self.as_mut_rgb32f().unwrap(), other),
            ColorType::Rgba32F => set_alpha_step_a(self.as_mut_rgba32f().unwrap(), other),
            _ => Err(Error::UnsupportedType),
        }?;
        Ok(())
    }
}
fn blend_step_a<Pmut, ContainerMut>(subject: &mut ImageBuffer<Pmut, ContainerMut>, other: &DynamicImage, op: fn(f64, f64) -> f64, apply_to_color: bool, apply_to_alpha: bool) -> Result<(), Error>
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
fn get_alpha_step_a<P, Container>(subject: &mut ImageBuffer<P, Container>) -> Result<(), Error>
where 
    P: Pixel,
    Container: DerefMut<Target = [P::Subpixel]> + AsRef<[<P as image::Pixel>::Subpixel]> + Clone,
{
    let alpha = subject.get_alpha().ok_or(Error::NoAlphaChannel)?;
    *subject = alpha;
    Ok(())
}
fn set_alpha_step_a<Pmut, ContainerMut>(subject: &mut ImageBuffer<Pmut, ContainerMut>, other: &DynamicImage) -> Result<(), Error>
where 
    Pmut: Pixel,
    ContainerMut: DerefMut<Target = [Pmut::Subpixel]>
    + DerefMut<Target = [Pmut::Subpixel]>
    + AsMut<[<Pmut as Pixel>::Subpixel]>,
{
    match other.color() {
        ColorType::L8 => subject.set_alpha(other.as_luma8().unwrap()),
        ColorType::La8 => subject.set_alpha(other.as_luma_alpha8().unwrap()),
        ColorType::Rgb8 => subject.set_alpha(other.as_rgb8().unwrap()),
        ColorType::Rgba8 => subject.set_alpha(other.as_rgba8().unwrap()),
        ColorType::L16 => subject.set_alpha(other.as_luma16().unwrap()),
        ColorType::La16 => subject.set_alpha(other.as_luma_alpha16().unwrap()),
        ColorType::Rgb16 => subject.set_alpha(other.as_rgb16().unwrap()),
        ColorType::Rgba16 => subject.set_alpha(other.as_rgba16().unwrap()),
        ColorType::Rgb32F => subject.set_alpha(other.as_rgb32f().unwrap()),
        ColorType::Rgba32F => subject.set_alpha(other.as_rgba32f().unwrap()),
        _ => Err(Error::UnsupportedType),
    }
}
fn transplant_alpha_step_a<Pmut, ContainerMut>(subject: &mut ImageBuffer<Pmut, ContainerMut>, other: &DynamicImage) -> Result<(), Error>
where 
    Pmut: Pixel,
    ContainerMut: DerefMut<Target = [Pmut::Subpixel]>
    + DerefMut<Target = [Pmut::Subpixel]>
    + AsMut<[<Pmut as Pixel>::Subpixel]>,
{
    match other.color() {
        ColorType::L8 => subject.transplant_alpha(other.as_luma8().unwrap()),
        ColorType::La8 => subject.transplant_alpha(other.as_luma_alpha8().unwrap()),
        ColorType::Rgb8 => subject.transplant_alpha(other.as_rgb8().unwrap()),
        ColorType::Rgba8 => subject.transplant_alpha(other.as_rgba8().unwrap()),
        ColorType::L16 => subject.transplant_alpha(other.as_luma16().unwrap()),
        ColorType::La16 => subject.transplant_alpha(other.as_luma_alpha16().unwrap()),
        ColorType::Rgb16 => subject.transplant_alpha(other.as_rgb16().unwrap()),
        ColorType::Rgba16 => subject.transplant_alpha(other.as_rgba16().unwrap()),
        ColorType::Rgb32F => subject.transplant_alpha(other.as_rgb32f().unwrap()),
        ColorType::Rgba32F => subject.transplant_alpha(other.as_rgba32f().unwrap()),
        _ => Err(Error::UnsupportedType),
    }
}
