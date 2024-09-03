// Tests
#[cfg(test)]
mod test {
    use std::iter;

    use crate::{
        pixelops::{
            pixel_add, pixel_darker, pixel_diff, pixel_div, pixel_hard_light, pixel_lighter,
            pixel_mult, pixel_overlay, pixel_screen, pixel_soft_light, pixel_sub,
        }, enums::{ColorStructure, ColorString},
        DynamicChops
    };
    use image::{open, DynamicImage};
    use rayon::prelude::{ParallelBridge, ParallelIterator};
    fn as_all_types(img: &DynamicImage) -> impl Iterator<Item = DynamicImage> {
        iter::once(DynamicImage::ImageLuma8(img.clone().into_luma8()))
            .chain(iter::once(DynamicImage::ImageLumaA8(
                img.clone().into_luma_alpha8(),
            )))
            .chain(iter::once(DynamicImage::ImageRgb8(img.clone().into_rgb8())))
            .chain(iter::once(DynamicImage::ImageRgba8(
                img.clone().into_rgba8(),
            )))
            .chain(iter::once(DynamicImage::ImageLuma16(
                img.clone().into_luma16(),
            )))
            .chain(iter::once(DynamicImage::ImageLumaA16(
                img.clone().into_luma_alpha16(),
            )))
            .chain(iter::once(DynamicImage::ImageRgb16(
                img.clone().into_rgb16(),
            )))
            .chain(iter::once(DynamicImage::ImageRgba16(
                img.clone().into_rgba16(),
            )))
            .chain(iter::once(DynamicImage::ImageRgb32F(
                img.clone().into_rgb32f(),
            )))
            .chain(iter::once(DynamicImage::ImageRgba32F(
                img.clone().into_rgba32f(),
            )))
    }
    type OpVec = Vec<(&'static str, fn(f64, f64) -> f64)>;
    fn all_pixel_ops() -> OpVec {
        vec![
            ("add", pixel_add),
            ("sub", pixel_sub),
            ("div", pixel_div),
            ("darker", pixel_darker),
            ("lighter", pixel_lighter),
            ("diff", pixel_diff),
            ("mult", pixel_mult),
            ("screen", pixel_screen),
            ("overlay", pixel_overlay),
            ("hard_light", pixel_hard_light),
            ("soft_light", pixel_soft_light),
        ]
    }
    #[test]
    fn test_dynamic() {
        let img1 = open("test_data/1.png").unwrap();
        let img2 = open("test_data/2.png").unwrap();
        as_all_types(&img1).par_bridge().for_each(|a| {
            let color_a = a.color().color_str();
            let structure_a: ColorStructure = a.color().into();
            as_all_types(&img2).par_bridge().for_each(|b| {
                let color_b = b.color().color_str();
                let structure_b: ColorStructure = b.color().into();
                let mut a_copy = a.clone();
                let res = a_copy.blend(&b, pixel_mult, true, true);
                match res {
                    Ok(()) => {
                        // Convert to rgb before saving as can't save some types
                        let out = DynamicImage::ImageRgba8(a_copy.into_rgba8());
                        out.save(format!(
                            "tests_out/dynamic_{color_a}_{color_b}.png",
                        ))
                        .unwrap();
                    }
                    Err(e) => {
                        // Should only error if a is L or La and b is Rgb or Rgba
                        assert!(!structure_a.rgb() && structure_b.rgb(), "{}", e);
                    }
                };
            });
        });
    }
    #[test]
    fn test_ops_alpha() {
        let img1 = open("test_data/1.png").unwrap();
        let img2 = open("test_data/2.png").unwrap();
        for do_color in [true, false] {
            for do_alpha in [true, false] {
                let blend_params = match (do_color, do_alpha) {
                    (true, true) => "colour_alpha",
                    (true, false) => "colour",
                    (false, true) => "alpha",
                    (false, false) => continue,
                };
                for (op_name, op) in all_pixel_ops() {
                    let mut img1_copy = img1.clone();
                    img1_copy.blend(&img2, op, do_color, do_alpha).unwrap();
                    img1_copy
                        .save(format!("tests_out/op_{op_name}_{blend_params}.png"))
                        .unwrap();
                }
            }
        }
    }
    #[test]
    fn test_ops() {
        let img1 = open("test_data/1_solid.png").unwrap();
        let img2 = open("test_data/2_solid.png").unwrap();
        for (op_name, op) in all_pixel_ops() {
            let mut img1_copy = img1.clone();
            img1_copy.blend(&img2, op, true, false).unwrap();
            img1_copy
                .save(format!("tests_out/solid_op_{op_name}.png"))
                .unwrap();
        }
    }
    #[test]
    fn test_overlay() {
        let img1 = open("test_data/1_solid.png").unwrap();
        let img2 = open("test_data/overlay.png").unwrap();
        for (op_name, op) in all_pixel_ops() {
            let mut img1_copy = img1.clone();
            img1_copy.blend(&img2, op, true, false).unwrap();
            img1_copy
                .save(format!("tests_out/overlay_{op_name}_ab.png"))
                .unwrap();

            let mut img2_copy = img2.clone();
            img2_copy.blend(&img1, op, true, false).unwrap();
            img2_copy
                .save(format!("tests_out/overlay_{op_name}_ba.png"))
                .unwrap();
        }
    }
    #[test]
    fn test_alpha_getters_n_setters() {
        let img1 = DynamicImage::ImageRgba8(open("test_data/1_solid.png").unwrap().to_rgba8());
        let img2 = DynamicImage::ImageRgba8(open("test_data/2.png").unwrap().to_rgba8());

        let img2alpha = img2.get_alpha().unwrap();
        img2alpha.clone().save("tests_out/alpha_get_alpha.png").unwrap();

        let mut img1_with_alpha = img1.clone();
        img1_with_alpha.set_alpha(&img2alpha).unwrap();
        img1_with_alpha
            .save("tests_out/alpha_set_alpha.png")
            .unwrap();

        let mut img1_transplant_alpha = img1.clone();
        img1_transplant_alpha.transplant_alpha(&img2).unwrap();
        img1_transplant_alpha
            .save("tests_out/alpha_transplant_alpha.png")
            .unwrap();
    }
    #[test]
    fn test_alpha_getters_n_setters_dynamics() {
        let img1 = open("test_data/1.png").unwrap();
        let img2 = open("test_data/2.png").unwrap();
        as_all_types(&img1).par_bridge().for_each(|a| {
            let color_a = a.color().color_str();
            let structure_a: ColorStructure = a.color().into();
            if !structure_a.alpha() {
                return;
            }
            let a_alpha = a.get_alpha().unwrap();
            as_all_types(&img2).par_bridge().for_each(|b| {
                let color_b = b.color().color_str();
                let structure_b: ColorStructure = b.color().into();
                if !structure_b.alpha() {
                    return;
                }
                let b_alpha = b.get_alpha().unwrap();

                let mut a_with_b = a.clone();
                a_with_b.set_alpha(&b_alpha).unwrap();

                let mut a_transplant_b = a.clone();
                a_transplant_b.transplant_alpha(&b).unwrap();

                let mut b_with_a = b.clone();
                b_with_a.set_alpha(&a_alpha).unwrap();

                let mut b_transplant_a = b.clone();
                b_transplant_a.transplant_alpha(&a).unwrap();

                DynamicImage::ImageRgba8(a_with_b.into_rgba8()).save(format!(
                    "tests_out/alpha_alltypes_{color_a}_set_{color_b}.png",
                )).unwrap();
                DynamicImage::ImageRgba8(a_transplant_b.into_rgba8()).save(format!(
                    "tests_out/alpha_alltypes_{color_a}_transplant_{color_b}.png",
                )).unwrap();
            });
            DynamicImage::ImageRgba8(a_alpha.into_rgba8()).save(format!(
                "tests_out/alpha_alltypes_{color_a}_alpha.png",
            )).unwrap();
        });
    }
}
