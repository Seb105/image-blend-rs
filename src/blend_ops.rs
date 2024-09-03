use std::{
    iter::{zip, Zip},
    ops::{Deref, DerefMut},
    vec,
};

use image::{GenericImageView, ImageBuffer, Pixel};
use num_traits::{Bounded, NumCast};

use crate::{
    enums::{ColorString, ColorStructure},
    error::Error,
};

pub(crate) fn dims_match<T: GenericImageView, U: GenericImageView>(a: &mut T, b: &U) -> Result<(), Error> {
    if (a.dimensions()) != b.dimensions() {
        return Err(Error::DimensionMismatch);
    }
    Ok(())
}
pub trait BufferBlend<P, Container>
where
    P: Pixel,
    Container: Deref<Target = [P::Subpixel]> + AsRef<[P::Subpixel]>,
{
    /**
    Blend `other` into `self` using the function `op`, where arg 0 is self and 1 is other.

    Handles type conversion and alpha channel detection and placement automatically.

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
    let mut img2_buffer = img2_dynamic.as_mut_rgba8().unwrap();
    img2_buffer.set_alpha(&img1_alpha).unwrap();
    img2_buffer.save("tests_out/doctest_buffer_getalpha_result.png").unwrap();

    ```
    */
    fn blend(
        &mut self,
        other: &ImageBuffer<P, Container>,
        op: fn(f64, f64) -> f64,
        apply_to_color: bool,
        apply_to_alpha: bool,
    ) -> Result<(), Error>;
}
impl<P, Pmut, Container, ContainerMut> BufferBlend<P, Container> for ImageBuffer<Pmut, ContainerMut>
where
    Pmut: Pixel,
    P: Pixel,
    Container: Deref<Target = [P::Subpixel]> + AsRef<[<P as Pixel>::Subpixel]>,
    ContainerMut: DerefMut<Target = [Pmut::Subpixel]>
        + DerefMut<Target = [Pmut::Subpixel]>
        + AsMut<[<Pmut as Pixel>::Subpixel]>,
{
    fn blend(
        &mut self,
        other: &ImageBuffer<P, Container>,
        op: fn(f64, f64) -> f64,
        apply_to_color: bool,
        apply_to_alpha: bool,
    ) -> Result<(), Error> {
        dims_match(self, other)?;
        let structure_a: ColorStructure = self.sample_layout().try_into()?;
        let structure_b: ColorStructure = other.sample_layout().try_into()?;

        let (colour_channels, alpha_channels) = get_channels(&structure_a, &structure_b)?;

        let a_max = type_max::<Pmut>();
        let b_max = type_max::<P>();

        if apply_to_color {
            zip(self.pixels_mut(), other.pixels()).for_each(|(px_a, px_b)| {
                let channel_a = px_a.channels_mut();
                let channel_b = px_b.channels();
                let alpha_weight = match structure_b.alpha_channel() {
                    Some(alpha_channel) => {
                        <f64 as NumCast>::from(channel_b[alpha_channel]).unwrap() / b_max
                    }
                    None => 1.,
                };
                if alpha_weight == 0. {
                    return;
                };
                colour_channels.clone().for_each(|(ch_a, ch_b)| {
                    let a_f64: f64 = <f64 as NumCast>::from(channel_a[ch_a]).unwrap() / a_max;
                    let b_f64: f64 = <f64 as NumCast>::from(channel_b[ch_b]).unwrap() / b_max;
                    let new_64_unweighted: f64 = NumCast::from(op(a_f64, b_f64)).unwrap();
                    let new_64 = new_64_unweighted * alpha_weight + a_f64 * (1. - alpha_weight);
                    let new_val = NumCast::from(new_64.clamp(0., 1.0) * a_max).unwrap();
                    channel_a[ch_a] = new_val;
                });
            });
        };
        if apply_to_alpha {
            if let Some((alpha_a, alpha_b)) = alpha_channels {
                zip(self.pixels_mut(), other.pixels()).for_each(|(px_a, px_b)| {
                    let channel_a = px_a.channels_mut();
                    let channel_b = px_b.channels();

                    let a_f64: f64 = <f64 as NumCast>::from(channel_a[alpha_a]).unwrap() / a_max;
                    let b_f64: f64 = <f64 as NumCast>::from(channel_b[alpha_b]).unwrap() / b_max;
                    let new_64: f64 = NumCast::from(op(a_f64, b_f64)).unwrap();
                    let new_val = NumCast::from(new_64.clamp(0., 1.0) * a_max).unwrap();
                    channel_a[alpha_a] = new_val;
                });
            }
        }

        Ok(())
    }
}

pub(crate) fn type_max<P>() -> f64 where P: Pixel {
    let max: f64 = NumCast::from(<P as Pixel>::Subpixel::max_value()).unwrap();
    let f32_max: f64 = NumCast::from(<f32 as Bounded>::max_value()).unwrap();
    // Hack to get around f32 images having a max value of 1.0 not f32::MAX
    if max - f32_max == 0. {
        return 1.
    }
    max
}

type ChannelIter = (
    Zip<vec::IntoIter<usize>, vec::IntoIter<usize>>,
    Option<(usize, usize)>,
);
fn get_channels(
    structure_a: &ColorStructure,
    structure_b: &ColorStructure,
) -> Result<ChannelIter, Error> {
    let colour_channels = match (structure_a.rgb(), structure_b.rgb()) {
        (true, true) => zip(vec![0usize, 1, 2], vec![0usize, 1, 2]),
        (true, false) => zip(vec![0, 1, 2], vec![0, 0, 0]),
        (false, false) => zip(vec![0], vec![0]),
        (false, true) => Err(Error::UnsupportedBlend(
            structure_a.color_str(),
            structure_b.color_str(),
        ))?,
    };
    let alpha_channels = match (structure_a.alpha(), structure_b.alpha()) {
        (true, true) => Some((
            structure_a.alpha_channel().unwrap(),
            structure_b.alpha_channel().unwrap(),
        )),
        _ => None,
    };
    Ok((colour_channels, alpha_channels))
}
