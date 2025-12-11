# embedded-graphics-colorcast

Color casting for [embedded-graphics] images.

This crate provides an `Image` struct that wraps around an
`ImageRaw<BinaryColor>` and allows rendering it to any draw target by mapping
the binary colors to the target's color type.

[embedded-graphics]: https://github.com/embedded-graphics/embedded-graphics

```rust
use embedded_graphics::{
    image::{ImageRaw, ImageRawBE},
    pixelcolor::{BinaryColor, Rgb565},
    prelude::*,
};
use embedded_graphics_colorcast::Image;

let mut display: Display<Rgb565> = Display::default();

// Raw big endian image data for demonstration purposes. A real image would likely be much
// larger.
let data = [
    0x00, 0x00, 0xF8, 0x00, 0x07, 0xE0, 0xFF, 0xE0, //
    0x00, 0x1F, 0x07, 0xFF, 0xF8, 0x1F, 0xFF, 0xFF, //
];

// Create a raw image instance. Other image formats will require different code to load them.
// All code after loading is the same for any image format.
let raw: ImageRawBE<BinaryColor> = ImageRaw::new(&data, 4);

// Create an `Image` object to position the image at `Point::zero()`.
let image = Image::new(&raw, Point::zero(), Rgb565::WHITE);

// Draw the image to the display.
image.draw(&mut display)?;
```

## Acknowledgements

This repository contains trait implementations and documentation copied over
from the `embedded-graphics` project (with very minor modifications).

## License

`MIT OR Apache-2.0`
