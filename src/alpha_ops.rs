use std::{iter::zip, ops::{Deref, DerefMut}};

use image::{imageops::grayscale, GenericImageView, ImageBuffer, Luma, Pixel};
use num_traits::{Bounded, NumCast};

use crate::{blend_ops::{dims_match, type_max}, enums::ColorStructure, error::Error};

pub trait GetAlpha<P, Container>
where
    P: Pixel,
    Container: DerefMut<Target = [P::Subpixel]> + AsRef<[<P as Pixel>::Subpixel]>
{
    #[allow(clippy::type_complexity)]
    fn get_alpha(
        &self
    ) -> Option<Self> where Self: std::marker::Sized;
}
impl<P, Container> GetAlpha<P, Container> for ImageBuffer<P, Container>
where
    P: Pixel,
    Container: DerefMut<Target = [P::Subpixel]> + AsRef<[<P as Pixel>::Subpixel]> + Clone
{
    // Get the alpha channel of an image as a grayscale image
    fn get_alpha(
        &self,
    ) -> Option<Self> {
        let color_structure: ColorStructure = self.sample_layout().try_into().ok()?;
        if !color_structure.alpha() {
            return None;
        }
        let colour_channels = if (color_structure.rgb()) {
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
            for ch in colour_channels.clone() {
                px_channels[ch] = alpha_val;
            }
            px_channels[alpha_channel] = max;
        });
        Some(alpha)
    }
}
pub trait SetAlpha<P, Container>
where
    P: Pixel,
    Container: Deref<Target = [P::Subpixel]> + AsRef<[P::Subpixel]>,
{
    // Set an image's alpha channel from a grayscale image
    // WARNING: Supports any type, but only the first channel will be used.
    fn set_alpha(
        &mut self,
        other: &ImageBuffer<P, Container>
    ) -> Result<(), Error>;
    // Transplant the alpha channel from one image to another
    fn transplant_alpha(
        &mut self,
        other: &ImageBuffer<P, Container>
    ) -> Result<(), Error>;
}
impl<P, Pmut, Container, ContainerMut> SetAlpha<P, Container> for ImageBuffer<Pmut, ContainerMut>
where
    Pmut: Pixel,
    P: Pixel,
    Container: Deref<Target = [P::Subpixel]> + AsRef<[<P as Pixel>::Subpixel]>,
    ContainerMut: DerefMut<Target = [Pmut::Subpixel]>
        + DerefMut<Target = [Pmut::Subpixel]>
        + AsMut<[<Pmut as Pixel>::Subpixel]>,
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
