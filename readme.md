
# image-blend
### Library to perform blending and alpha channel operations using the image crate

Implementation of support for type-agnostic blending algorithms such as screen, multiply, lighter, etc, for the [image](https://crates.io/crates/image) crate

Also provide support for getting alpha channnels as grayscale images, setting alpha channels from grayscale images, and transplanting alpha chnnales

#### Type-agnostic: this library will automatically convert between input type when blending two images together.

The only limitation to this is that you cannot blend an Rgb/Rgba image into a Luma image.

## Usage:

Syntax is the same when working with Dynamic and Imagebuffer.

### Working with dynamic images
#### Blend two images together

```rust
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

#### Get and set the alpha channels

```rust
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

#### Transplant an alpha channel directly from one image to another

```rust
use image::open;
use image_blend::DynamicChops;

// Load an image and get its alpha channel
let img1_dynamic = open("test_data/1.png").unwrap();

// Load another image and set its alpha channel to a copy of the first image's alpha channel.
let mut img2_dynamic = open("test_data/2.png").unwrap();
img2_dynamic.transplant_alpha(&img1_dynamic).unwrap();
img2_dynamic.save("tests_out/doctest_dynamic_transplantalpha_result.png").unwrap();
```

### Working with imagebuffers

Note how in these examples, the image buffers have different types but it doesn't matter as the library handles this.

#### Blend two images together

```rust
use image::open;
use image_blend::BufferBlend;
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

#### Get and set alpha channels

```rust
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

#### Transplant an alpha channel directly from one image to another

```rust
use image::open;
use image_blend::{BufferGetAlpha, BufferSetAlpha};

// Load an image and get its alpha channel
let img1_dynamic = open("test_data/1.png").unwrap();
let img1_buffer = img1_dynamic.as_rgba8().unwrap();

// Load another image and set its alpha channel to a copy of the first image's alpha channel.
let mut img2_dynamic = open("test_data/2.png").unwrap();
let mut img2_buffer = img2_dynamic.to_rgba16();
img2_buffer.transplant_alpha(&img1_buffer).unwrap();
img2_buffer.save("tests_out/doctest_buffer_transplantalpha_result.png").unwrap();
```

## Custom blend operations

Using custom blend operations is easy. You just need a function that takes 2 f64s and returns an f64.

The values passed to this function are `0..1` where `0.` is the darkest a pixel can be and `1.` the brightest. Type conversion and clamping of the return to `0..1` is handled for you.

`a` is self, `b` is other.

Note that the return is cla

```rust
use image::open;
use image_blend::DynamicChops;

let closest_to_grey = |a: f64, b: f64| {
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
img1_dynamic.blend(&img2_dynamic, closest_to_grey, true, false).unwrap();
img1_dynamic.save("tests_out/doctest_dynamic_custom_result.png").unwrap();

```
