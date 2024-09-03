
# image-blend
### Library to perform blending and alpha channel operations using the image crate

Implementation of support for type-agnostic blending algorithms such as screen, multiply, lighter, etc, for the [image](https://crates.io/crates/image) crate

Also provide support for getting alpha channnels as grayscale images, setting alpha channels from grayscale images, and transplanting alpha chnnales

#### Type-agnostic: this library will automatically convert between input type when blending two images together.

The only limitation to this is that you cannot blend an Rgb/Rgba image into a Luma image.

## Usage:
### Working with dynamic images
#### Blending operations
```rust
todo
```
