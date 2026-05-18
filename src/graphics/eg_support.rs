#![cfg(feature = "eg")]

use crate::graphics::layer::Layer;
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    pixelcolor::{raw::{RawU16, RawData}, Rgb565},
    Pixel,
};
use core::convert::Infallible;

impl OriginDimensions for Layer {
    fn size(&self) -> Size {
        Size::new(self.width() as u32, self.height() as u32)
    }
}

impl DrawTarget for Layer {
    type Color = Rgb565;
    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let buf_ptr = self.buffer_ptr() as *mut u16;
        let width = self.width();
        let height = self.height();

        for Pixel(coord, color) in pixels.into_iter() {
            let x = coord.x;
            let y = coord.y;

            if x >= 0 && x < width && y >= 0 && y < height {
                let index = (y * width + x) as usize;
                
                let raw_color = RawU16::from(color).into_inner();

                unsafe {
                    *buf_ptr.add(index) = raw_color;
                }
            }
        }

        Ok(())
    }
}