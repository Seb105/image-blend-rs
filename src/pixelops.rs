/*!
This module contains functions for performing pixel operations.

All arguments and returns are f64 values in the range 0.0..1.0.

`a` is self, `b` is the other pixel.

Returns are not bounded in these functions, but are clamped to 0.0..1.0 before being converted back to the input type in the blend trait.

Formulas taken from [Wikipedia](https://en.wikipedia.org/wiki/Blend_modes).

Analagous blend modes of the same name in Photoshop.

# Examples

```
use image::open;
use image_blend::{BufferBlend};
use image_blend::pixelops::pixel_mult;

// Load an image
let mut img1_dynamic = open("test_data/1.png").unwrap();
let mut img1_buffer = img1_dynamic.as_mut_rgba8().unwrap();

// Load another image
let img2_dynamic = open("test_data/2.png").unwrap();
let img2_buffer = img2_dynamic.to_rgba16();

// Blend the images using the pixel_mult function
img1_buffer.blend(&img2_buffer, pixel_mult, true, false).unwrap();
img1_buffer.save("tests_out/doctest_buffer_blend_result.png").unwrap();

```
*/

/// Adds `a` to `b`.
#[must_use]
pub fn pixel_add(a: f64, b: f64) -> f64 {
    a + b
}

/// Subtracts `b` from `a`.
#[must_use]
pub fn pixel_sub(a: f64, b: f64) -> f64 {
    a - b
}

/// Divides `a` by `b`. If `b` is 0, returns 1.
#[must_use]
pub fn pixel_div(a: f64, b: f64) -> f64 {
    if b == 0. {
        return 1.;
    }
    a / b
}

/// Returns the darker value between `a` and `b`.
#[must_use]
pub fn pixel_darker(a: f64, b: f64) -> f64 {
    a.min(b)
}

/// Returns the lighter value between `a` and `b`.
#[must_use]
pub fn pixel_lighter(a: f64, b: f64) -> f64 {
    a.max(b)
}

/// Returns the absolute difference between `a` and `b`.
#[must_use]
pub fn pixel_diff(a: f64, b: f64) -> f64 {
    (a - b).abs()
}

/// Multiplies `a` by `b`.
#[must_use]
pub fn pixel_mult(a: f64, b: f64) -> f64 {
    a * b
}

/// Applies the screen blend mode to `a` and `b`.
#[must_use]
pub fn pixel_screen(a: f64, b: f64) -> f64 {
    1.0 - (1.0 - a) * (1.0 - b)
}

/// Applies the overlay blend mode to `a` and `b`.
#[must_use]
pub fn pixel_overlay(a: f64, b: f64) -> f64 {
    if a < 0.5 {
        2.0 * a * b
    } else {
        1.0 - 2.0 * (1.0 - a) * (1.0 - b)
    }
}

/// Applies the hard light blend mode to `a` and `b`.
#[must_use]
pub fn pixel_hard_light(a: f64, b: f64) -> f64 {
    if b < 0.5 {
        2.0 * a * b
    } else {
        1.0 - 2.0 * (1.0 - a) * (1.0 - b)
    }
}

/// Applies the soft light blend mode to `a` and `b`.
#[must_use]
pub fn pixel_soft_light(a: f64, b: f64) -> f64 {
    if b <= 0.5 {
        a - (1.0 - 2.0 * b) * a * (1.0 - a)
    } else {
        let gwc3 = if a <= 0.25 {
            ((16.0 * a - 12.0) * a + 4.0) * a
        } else {
            a.sqrt()
        };
        a + (2.0 * b - 1.0) * (gwc3 - a)
    }
}

/// Returns `b`.
#[must_use]
pub fn pixel_paste(_a: f64, b: f64) -> f64 {
    b
}
