use std::ops::DerefMut;

use image::{ColorType, DynamicImage, ImageBuffer, Pixel};

use crate::{BufferBlend, BufferGetAlpha, BufferSetAlpha, Error};

pub trait DynamicChops {
    /**
    Blend `other` into `self` using the function `op`, where arg 0 is self and 1 is other.

    Handles type conversion and alpha channel detection and placement automatically.

    If `other` has an alpha channel, it will be used to weight the blending of the color channels. If there is no alpha channel, the blending will be unweighted.

    You may blend a luma image into an rgba image (in which case the luma image will be treated as a grayscale rgb image), but you cannot blend an rgba image into a luma image.

    # Arguments

    Use `apply_to_color` and `apply_to_alpha` to control which channels are affected.

    If `apply_to_alpha` is true but `self` or `other` does not have an alpha channel, nothing will happen.

    `op` is a function that takes two f64 values and returns a f64 value. (e.g. `|self, other| self + other`)

    Standard blend modes such as those found in photoshop are provided as functions (e.g. `pixel_add`, `pixel_mult`, etc.).

    The values are normalized to the range 0.0..1.0 before blending, and then scaled back to the input type's range.

    The output from `op` is automatically clamped from 0.0..1.0 before being converted back to the input type so you don't need to worry about overflow/underflow.

    # Errors

    `DimensionMismatch`: `self` and `other` have different dimensions

    `UnsupportedBlend`: `self` is a luma image and `other` is an rgb image

    # Examples

    ## Example 1:

    Using the `pixel_mult` function to blend two images together:
    ```
    use image::open;
    use image_blend::DynamicChops;
    use image_blend::pixelops::pixel_mult;

    // Load an image
    let mut img1_dynamic = open("test_data/1.png").unwrap();

    // Load another image
    let img2_dynamic = open("test_data/2.png").unwrap();

    // Blend the images using the pixel_mult function
    img1_dynamic.blend(&img2_dynamic, pixel_mult, true, false).unwrap();
    img1_dynamic.save("tests_out/doctest_dynamic_blend_result.png").unwrap();

    ```
    ## Example 2:

    Using a custom function to blend two images together:

    ```
    use image::open;
    use image_blend::DynamicChops;

    let closest_to_gray = |a: f64, b: f64| {
        let a_diff = (a - 0.5).abs();
        let b_diff = (b - 0.5).abs();
        if a_diff < b_diff {
            a
        } else {
            b
        }
    };

    // Load an image
    let mut img1_dynamic = open("test_data/1.png").unwrap();

    // Load another image
    let img2_dynamic = open("test_data/2.png").unwrap();

    // Blend the images using our custom function
    img1_dynamic.blend(&img2_dynamic, closest_to_gray, true, false).unwrap();
    img1_dynamic.save("tests_out/doctest_dynamic_custom_result.png").unwrap();

    ```
    */
    fn blend (
        &mut self,
        other: &Self,
        op: fn(f64, f64) -> f64,
        apply_to_color: bool,
        apply_to_alpha: bool,
    ) -> Result<(), Error>;
    /**
    Get the alpha channel of this image as a grayscale with the same number of channels as the input image. (i.e a 3 channel rgb image will return a 3 channel rgb grayscale image)

    The alpha channel of the returned image is set to the maximum value of the input type.

    If the image does not have an alpha channel, return None.


    # Examples

    ```
    use image::open;
    use image_blend::DynamicChops;

    // Load an image and get its alpha channel
    let img1_dynamic = open("test_data/1.png").unwrap();
    let img1_alpha = img1_dynamic.get_alpha().unwrap();
    img1_alpha.clone().save("tests_out/doctest_dynamic_getalpha_alpha.png").unwrap();

    // Load another image and set its alpha channel to the first image's alpha channel, using the copied alpha channel
    let mut img2_dynamic = open("test_data/2.png").unwrap();
    img2_dynamic.set_alpha(&img1_alpha).unwrap();
    img2_dynamic.save("tests_out/doctest_dynamic_getalpha_result.png").unwrap();

    ```
    */
    fn get_alpha(
        &self,
    ) -> Option<Self> where Self: std::marker::Sized;
    /**
    Set an image's alpha channel from another images alpha channel. 

    Handles type conversion and alpha channel placement automatically.

    # Errors
    `NoAlphaChannel`: `self` or `other` does not have an alpha channel

    `DimensionMismatch`: `self` and `other` have different dimensions


    # Examples

    ```
    use image::open;
    use image_blend::DynamicChops;

    // Load an image and get its alpha channel
    let img1_dynamic = open("test_data/1.png").unwrap();

    // Load another image and set its alpha channel to a copy of the first image's alpha channel.
    let mut img2_dynamic = open("test_data/2.png").unwrap();
    img2_dynamic.transplant_alpha(&img1_dynamic).unwrap();
    img2_dynamic.save("tests_out/doctest_dynamic_transplantalpha_result.png").unwrap();
    ```
    */
    fn transplant_alpha(
        &mut self,
        other: &Self
    ) -> Result<(), Error>;
    /**
    Set an image's alpha channel using the grascale color of another image. 

    Handles type conversion and alpha channel detection and placement automatically.

    WARNING: `other` can be of any type, but only the first channel will be used to set the alpha channel.

    # Errors
    `NoAlphaChannel`: `self` does not have an alpha channel

    `DimensionMismatch`: `self` and `other` have different dimensions


    # Examples

    ```
    use image::open;
    use image_blend::DynamicChops;

    // Load an image and get its alpha channel
    let img1_dynamic = open("test_data/1.png").unwrap();
    let img1_alpha = img1_dynamic.get_alpha().unwrap();
    img1_alpha.clone().save("tests_out/doctest_dynamic_setalpha_alpha.png").unwrap();

    // Load another image and set its alpha channel to the first image's alpha channel, using the copied alpha channel
    let mut img2_dynamic = open("test_data/2.png").unwrap();
    img2_dynamic.set_alpha(&img1_alpha).unwrap();
    img2_dynamic.save("tests_out/doctest_dynamic_setalpha_result.png").unwrap();

    ```
    */
    fn set_alpha(
        &mut self,
        other: &Self
    ) -> Result<(), Error> where Self: std::marker::Sized;
}
impl DynamicChops for DynamicImage {
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
    ) -> Option<DynamicImage> {
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
        }.ok()?;
        Some(copy)
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
