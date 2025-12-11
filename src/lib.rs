//! Color casting for embedded-graphics images.
//!
//! This crate provides an `Image` struct that wraps around an
//! `ImageRaw<BinaryColor>` and allows rendering it to any draw target by mapping the
//! binary colors to the target's color type.
//!
//! # Examples
//!
//! ```rust
//! use embedded_graphics::{
//!     image::{ImageRaw, ImageRawBE},
//!     pixelcolor::{BinaryColor, Rgb565},
//!     prelude::*,
//! };
//! use embedded_graphics_colorcast::Image;
//! # use embedded_graphics::mock_display::MockDisplay as Display;
//!
//! let mut display: Display<Rgb565> = Display::default();
//!
//! // Raw big endian image data for demonstration purposes. A real image would likely be much
//! // larger.
//! let data = [
//!     0x00, 0x00, 0xF8, 0x00, 0x07, 0xE0, 0xFF, 0xE0, //
//!     0x00, 0x1F, 0x07, 0xFF, 0xF8, 0x1F, 0xFF, 0xFF, //
//! ];
//!
//! // Create a raw image instance. Other image formats will require different code to load them.
//! // All code after loading is the same for any image format.
//! let raw: ImageRawBE<BinaryColor> = ImageRaw::new(&data, 4);
//!
//! // Create an `Image` object to position the image at `Point::zero()`.
//! let image = Image::new(&raw, Point::zero(), Rgb565::WHITE);
//!
//! // Draw the image to the display.
//! image.draw(&mut display)?;
//!
//! # Ok::<(), core::convert::Infallible>(())
//! ```
#![no_std]

use embedded_graphics::{
    Drawable, Pixel,
    geometry::OriginDimensions,
    image::GetPixel,
    pixelcolor::BinaryColor,
    prelude::{Dimensions, DrawTarget, PixelColor, Point, PointsIter, Transform},
    primitives::Rectangle,
};

/// Image object.
///
/// The `Image` struct is a wrapper around an [`ImageRaw<BinaryColor>`] and can be rendered
/// to any draw target by mapping the binary colors to the target's color type.
///
/// This takes ownership of the `ImageRaw` since it's only holding a reference,
/// not the entire image data.
#[derive(Debug, Clone, Copy)]
pub struct Image<'a, T, C>
where
    T: OriginDimensions + GetPixel<Color = BinaryColor>,
    C: PixelColor,
{
    image: &'a T,
    position: Point,
    color: C,
}

impl<'a, T, C> Image<'a, T, C>
where
    T: OriginDimensions + GetPixel<Color = BinaryColor>,
    C: PixelColor,
{
    /// Create a new `Image` at a given position
    pub const fn new(image: &'a T, position: Point, color: C) -> Self {
        Self {
            image,
            position,
            color,
        }
    }

    /// Create a new `Image` centered around a given point
    pub fn with_center(image: &'a T, center: Point, color: C) -> Self {
        let position = Rectangle::with_center(center, image.size()).top_left;
        Self {
            image,
            position,
            color,
        }
    }
}

impl<T, C> Drawable for Image<'_, T, C>
where
    T: OriginDimensions + GetPixel<Color = BinaryColor>,
    C: PixelColor,
{
    type Color = C;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        target.draw_iter(self.image.bounding_box().points().flat_map(|point| {
            if self.image.pixel(point) == Some(BinaryColor::On) {
                Some(Pixel(self.position + point, self.color))
            } else {
                None
            }
        }))
    }
}

impl<T, C> Transform for Image<'_, T, C>
where
    T: OriginDimensions + GetPixel<Color = BinaryColor>,
    C: PixelColor,
{
    /// Translate the image by a given delta, returning a new image
    ///
    /// # Examples
    ///
    /// ## Move an image around
    ///
    /// This examples moves a 4x4 black and white image by `(10, 20)` pixels without mutating the
    /// original image
    ///
    /// ```rust
    /// use embedded_graphics::{
    ///     geometry::Point,
    ///     image::{Image, ImageRaw},
    ///     pixelcolor::BinaryColor,
    ///     prelude::*,
    /// };
    ///
    /// let image: ImageRaw<BinaryColor> = ImageRaw::new(&[0xff, 0x00, 0xff, 0x00], 4);
    ///
    /// let image = Image::new(&image, Point::zero());
    ///
    /// let image_moved = image.translate(Point::new(10, 20));
    ///
    /// assert_eq!(image.bounding_box().top_left, Point::zero());
    /// assert_eq!(image_moved.bounding_box().top_left, Point::new(10, 20));
    /// ```
    fn translate(&self, by: Point) -> Self {
        Self {
            image: self.image,
            position: self.position + by,
            color: self.color,
        }
    }

    /// Translate the image by a given delta, modifying the original object
    ///
    /// # Examples
    ///
    /// ## Move an image around
    ///
    /// This examples moves a 4x4 black and white image by `(10, 20)` pixels by mutating the
    /// original image
    ///
    /// ```rust
    /// use embedded_graphics::{
    ///     geometry::Point,
    ///     image::{Image, ImageRaw},
    ///     pixelcolor::BinaryColor,
    ///     prelude::*,
    /// };
    ///
    /// let image: ImageRaw<BinaryColor> = ImageRaw::new(&[0xff, 0x00, 0xff, 0x00], 4);
    ///
    /// let mut image = Image::new(&image, Point::zero());
    ///
    /// image.translate_mut(Point::new(10, 20));
    ///
    /// assert_eq!(image.bounding_box().top_left, Point::new(10, 20));
    /// ```
    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.position += by;

        self
    }
}

impl<T, C> Dimensions for Image<'_, T, C>
where
    T: OriginDimensions + GetPixel<Color = BinaryColor>,
    C: PixelColor,
{
    fn bounding_box(&self) -> Rectangle {
        self.image.bounding_box().translate(self.position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_graphics::{image::ImageRaw, pixelcolor::Rgb666, prelude::RgbColor};

    #[test]
    fn test_image_from_imageraw() {
        let image_raw = ImageRaw::<BinaryColor>::new(&[0b10101010, 0b01010101], 8);
        Image::new(&image_raw, Point::zero(), Rgb666::WHITE);
    }

    #[test]
    fn test_image_from_imageraw_inverted() {
        let image_raw = ImageRaw::<BinaryColor>::new(&[0b10101010, 0b01010101], 8);
        Image::new(&image_raw, Point::zero(), BinaryColor::Off);
    }
}
