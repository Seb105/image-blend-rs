use std::{iter::{zip, Zip}, ops::{Deref, DerefMut}, vec};

use image::{GenericImageView, ImageBuffer, Pixel};
use num_traits::{NumCast, Bounded};

use crate::{enums::{ColorString, ColorStructure}, error::Error};

fn check_dims<T: GenericImageView, U: GenericImageView>(a: &mut T, b: &U) -> Result<(), Error>
{
    if (a.dimensions()) != b.dimensions() {
        return Err(Error::DimensionMismatch);
    }
    Ok(())
}

pub fn blend<P, Pmut, Container, ContainerMut>(a: &mut ImageBuffer<Pmut, ContainerMut>, b: &ImageBuffer<P, Container>, op: fn(f64, f64) -> f64) -> Result<(), Error>
where
    Pmut: Pixel,
    P: Pixel,
    Container: Deref<Target=[P::Subpixel]> + AsRef<[<P as Pixel>::Subpixel]>,
    ContainerMut: DerefMut<Target=[Pmut::Subpixel]> + DerefMut<Target=[Pmut::Subpixel]> + AsMut<[<Pmut as Pixel>::Subpixel]>
{
    check_dims(a, b)?;

    let layout_a = a.sample_layout();
    let layout_b = b.sample_layout();

    let structure_a: ColorStructure = layout_a.try_into()?;
    let structure_b: ColorStructure = layout_b.try_into()?;

    let (colour_channels, alpha_channels) = get_channels(structure_a, structure_b)?;

    let a_max: f64 = NumCast::from(<Pmut as Pixel>::Subpixel::max_value()).unwrap();
    let b_max: f64 = NumCast::from(<P as Pixel>::Subpixel::max_value()).unwrap();

    zip(a.pixels_mut(),b.pixels()).for_each(|(px_a, px_b)| {
        let channel_a = px_a.channels_mut();
        let channel_b = px_b.channels();

        colour_channels.clone().for_each(|(ch_a, ch_b)| {
            let a_f64: f64 = <f64 as NumCast>::from(channel_a[ch_a]).unwrap() / a_max;
            let b_f64: f64 = <f64 as NumCast>::from(channel_b[ch_b]).unwrap() / b_max;
            let new_val = NumCast::from((op(a_f64, b_f64)).min(1.0).max(0.) * a_max).unwrap();
            channel_a[ch_a] = new_val;
        });
    });
    if let Some((alpha_a, alpha_b)) = alpha_channels {
        zip(a.pixels_mut(), b.pixels()).for_each(|(px_a, px_b)| {
            let channel_a = px_a.channels_mut();
            let channel_b = px_b.channels();

            let a_f64: f64 = <f64 as NumCast>::from(channel_a[alpha_a]).unwrap() / a_max;
            let b_f64: f64 = <f64 as NumCast>::from(channel_b[alpha_b]).unwrap() / b_max;
            let new_val = NumCast::from((op(a_f64, b_f64)).min(1.0).max(0.) * a_max).unwrap();
            channel_a[alpha_a] = new_val;
        });
    }

    Ok(())
}

fn get_channels(structure_a: ColorStructure, structure_b: ColorStructure) -> Result<(Zip<vec::IntoIter<usize>, vec::IntoIter<usize>>, Option<(usize, usize)>), Error> 
{
    let colour_channels = match (structure_a.rgb(), structure_b.rgb()) {
        (true, true) => zip(vec![0usize, 1, 2], vec![0usize, 1, 2]),
        (true, false) => zip(vec![0, 1, 2], vec![0, 0, 0]),
        (false, false) => zip(vec![0], vec![0]),
        (false, true) => Err(Error::UnsupportedBlend(structure_a.color_str(), structure_b.color_str()))?,
    };
    let alpha_channels = match (structure_a.alpha(), structure_b.alpha()) {
        (true, true) => Some((structure_a.alpha_channel().unwrap(), structure_b.alpha_channel().unwrap())),
        _ => None,
    };
    Ok((colour_channels, alpha_channels))
}
