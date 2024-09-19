use std::{iter::zip, ops::{Deref, DerefMut}};

use image::{ImageBuffer, Pixel};
use num_traits::{Bounded, NumCast};

use crate::{blend_ops::{dims_match, type_max}, enums::ColorStructure, error::Error};

pub trait BufferGetAlpha<P, Container>
where
    P: Pixel,
    Container: DerefMut<Target = [P::Subpixel]> + AsRef<[<P as Pixel>::Subpixel]>
{
    /**
    Get the alpha channel of this image as a grayscale with the same number of channels as the input image. (i.e a 4 channel rgba image will return a 3 channel rgba grayscale image)

    The alpha channel of the returned image is set to the maximum value of the input type.

    If the image does not have an alpha channel, return None.


    # Examples

    ```
    use image::open;
    use image_blend::{BufferGetAlpha, BufferSetAlpha};

    // Load an image and get its alpha channel
    let img1_dynamic = open("test_data/1.png").unwrap();
    let img1_buffer = img1_dynamic.as_rgba8().unwrap();
    let img1_alpha = img1_buffer.get_alpha().unwrap();
    img1_alpha.clone().save("tests_out/doctest_buffer_getalpha_alpha.png").unwrap();

    // Load another image and set its alpha channel to the first image's alpha channel, using the copied alpha channel
    let mut img2_dynamic = open("test_data/2.png").unwrap();
    let mut img2_buffer = img2_dynamic.to_rgba16();
    img2_buffer.set_alpha(&img1_alpha).unwrap();
    img2_buffer.save("tests_out/doctest_buffer_getalpha_result.png").unwrap();

    ```
    */
    fn get_alpha(
        &self
    ) -> Option<Self> where Self: std::marker::Sized;
}
impl<P, Container> BufferGetAlpha<P, Container> for ImageBuffer<P, Container>
where
    P: Pixel,
    Container: DerefMut<Target = [P::Subpixel]> + AsRef<[<P as Pixel>::Subpixel]> + Clone
{
    fn get_alpha(
        &self,
    ) -> Option<Self> {
        let color_structure: ColorStructure = self.sample_layout().try_into().ok()?;
        if !color_structure.alpha() {
            return None;
        }
        let color_channels = if color_structure.rgb() {
            vec![0, 1, 2]
        } else {
            vec![0]
        };
        let alpha_channel = color_structure.alpha_channel().unwrap();
        let mut alpha = self.clone();

        let max: <P as Pixel>::Subpixel = NumCast::from(type_max::<P>()).unwrap();
        zip(alpha.pixels_mut(), self.pixels()).for_each(|(px_luma, px)| {
            // Don't need to cast here because we know the types are the same
            let alpha_val = px.channels()[alpha_channel];
            let px_channels = px_luma.channels_mut();
            for ch in color_channels.clone() {
                px_channels[ch] = alpha_val;
            }
            px_channels[alpha_channel] = max;
        });
        Some(alpha)
    }
}
pub trait BufferSetAlpha<P, Container>
where
    P: Pixel,
    Container: Deref<Target = [P::Subpixel]> + AsRef<[P::Subpixel]>,
{
    /**
    Set an image's alpha channel using the grascale color of another image. 

    Handles type conversion and alpha channel detection and placement automatically.

    WARNING: `other` can be of any type, but only the first channel will be used to set the alpha channel. In a grayscale image this will be the luma channel, in an rgb image the red channel. Consider converting to grayscale if this matters. In a grayscale image this will be the luma channel, in an rgb image the red channel.

    # Errors
    `NoAlphaChannel`: `self` does not have an alpha channel

    `DimensionMismatch`: `self` and `other` have different dimensions


    # Examples

    ```
    use image::open;
    use image_blend::{BufferGetAlpha, BufferSetAlpha};

    // Load an image and get its alpha channel
    let img1_dynamic = open("test_data/1.png").unwrap();
    let img1_buffer = img1_dynamic.as_rgba8().unwrap();
    let img1_alpha = img1_buffer.get_alpha().unwrap();
    img1_alpha.clone().save("tests_out/doctest_buffer_setalpha_alpha.png").unwrap();

    // Load another image and set its alpha channel to the first image's alpha channel, using the copied alpha channel
    let mut img2_dynamic = open("test_data/2.png").unwrap();
    let mut img2_buffer = img2_dynamic.to_rgba16();
    img2_buffer.set_alpha(&img1_alpha).unwrap();
    img2_buffer.save("tests_out/doctest_buffer_setalpha_result.png").unwrap();

    ```
    */
    fn set_alpha(
        &mut self,
        other: &ImageBuffer<P, Container>
    ) -> Result<(), Error>;
    
    /**
    Set an image's alpha channel from another images alpha channel. 

    Handles type conversion and alpha channel placement automatically.

    # Errors
    `NoAlphaChannel`: `self` or `other` does not have an alpha channel

    `DimensionMismatch`: `self` and `other` have different dimensions


    # Examples

    ```
    use image::open;
    use image_blend::{BufferGetAlpha, BufferSetAlpha};

    // Load an image that has an alpha channel
    let img1_dynamic = open("test_data/1.png").unwrap();
    let img1_buffer = img1_dynamic.as_rgba8().unwrap();

    // Load another image and set its alpha channel to a copy of the first image's alpha channel.
    let mut img2_dynamic = open("test_data/2.png").unwrap();
    let mut img2_buffer = img2_dynamic.to_rgba16();
    img2_buffer.transplant_alpha(&img1_buffer).unwrap();
    img2_buffer.save("tests_out/doctest_buffer_transplantalpha_result.png").unwrap();
    ```
    */
    fn transplant_alpha(
        &mut self,
        other: &ImageBuffer<P, Container>
    ) -> Result<(), Error>;
}
impl<P, Pmut, Container, ContainerMut> BufferSetAlpha<P, Container> for ImageBuffer<Pmut, ContainerMut>
where
    Pmut: Pixel,
    P: Pixel,
    Container: Deref<Target = [P::Subpixel]> + AsRef<[<P as Pixel>::Subpixel]>,
    ContainerMut: DerefMut<Target = [Pmut::Subpixel]>
        + DerefMut<Target = [Pmut::Subpixel]>
        + AsMut<[<Pmut as Pixel>::Subpixel]>
{
    fn set_alpha(
        &mut self,
        other: &ImageBuffer<P, Container>,
    ) -> Result<(), Error> {
        dims_match(self, other)?;
        let structure_a: ColorStructure = self.sample_layout().try_into()?;
        let alpha_channel = structure_a.alpha_channel().ok_or(Error::NoAlphaChannel)?;

        let a_max = type_max::<Pmut>();
        let b_max = type_max::<P>();

        zip(self.pixels_mut(), other.pixels()).for_each(|(px, px_luma)| {
            // Need to cast here because there is no guarantee P and Pmut are the same type
            let px_luma_64: f64 = <f64 as NumCast>::from(px_luma.channels()[0]).unwrap() / b_max;
            let alpha: <Pmut as Pixel>::Subpixel = NumCast::from(px_luma_64 * a_max).unwrap();
            px.channels_mut()[alpha_channel] = alpha;
        });
        Ok(())
    }
    fn transplant_alpha(
        &mut self,
        other: &ImageBuffer<P, Container>
    ) -> Result<(), Error> {
        dims_match(self, other)?;
        let structure_a: ColorStructure = self.sample_layout().try_into()?;
        let structure_b: ColorStructure = other.sample_layout().try_into()?;

        let alpha_a = structure_a.alpha_channel().ok_or(Error::NoAlphaChannel)?;
        let alpha_b = structure_b.alpha_channel().ok_or(Error::NoAlphaChannel)?;

        let a_max = type_max::<Pmut>();
        let b_max = type_max::<P>();

        zip(self.pixels_mut(), other.pixels()).for_each(|(pxa, pxb)| {
            // Need to cast here because there is no guarantee P and Pmut are the same type
            let float_b: f64 = <f64 as NumCast>::from(pxb.channels()[alpha_b]).unwrap() / b_max;
            let alpha: <Pmut as Pixel>::Subpixel = NumCast::from(float_b * a_max).unwrap();
            pxa.channels_mut()[alpha_a] = alpha;
        });
        Ok(())
    }
}
pub trait BufferStripAlpha<Pmut, ContainerMut> 
where 
    Pmut: Pixel, 
    ContainerMut: DerefMut<Target = [Pmut::Subpixel]> 
        + AsMut<[<Pmut as Pixel>::Subpixel]> 
        
{
    /**
    Remove this images alpha channel by setting it to the maximum value for every pixel.

    Does not modify the underlying type.


    # Errors
    `NoAlphaChannel`: `self` does not have an alpha channel


    # Examples

    ```
    use image::open;
    use image_blend::{BufferStripAlpha};

    // Load an image and remove its alpha channel
    let mut img2_dynamic = open("test_data/2.png").unwrap();
    let mut img2_buffer = img2_dynamic.to_rgba16();
    img2_buffer.strip_alpha().unwrap();
    img2_buffer.save("tests_out/doctest_buffer_stripalpha_result.png").unwrap();
    ```
    */
    fn strip_alpha(
        &mut self
    ) -> Result<(), Error>;
}
impl <Pmut, ContainerMut> BufferStripAlpha<Pmut, ContainerMut> for ImageBuffer<Pmut, ContainerMut>
where 
    Pmut: Pixel, 
    ContainerMut: DerefMut<Target = [Pmut::Subpixel]> 
        + AsMut<[<Pmut as Pixel>::Subpixel]> 
{
    fn strip_alpha(
            &mut self
    ) -> Result<(), Error> {
        let structure: ColorStructure = self.sample_layout().try_into()?;
        let alpha_channel = structure.alpha_channel().ok_or(Error::NoAlphaChannel)?;
        let max = <Pmut as Pixel>::Subpixel::max_value();
        self.pixels_mut().for_each(|px| {
            px.channels_mut()[alpha_channel] = max;
        });
        Ok(())
    }
}
