use bootloader_api::info::{FrameBuffer, PixelFormat};
use embedded_graphics::{
    Pixel,
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    pixelcolor::{Rgb888, RgbColor},
};
use core::convert::Infallible;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub fn set_px_pos(buf: &mut FrameBuffer, pos: Position, color: Color) {
    let info = buf.info();

    let offset = {
        let lineoff = pos.y * info.stride;
        let pxoff = lineoff + pos.x;
        pxoff * info.bytes_per_pixel
    };

    let pixel_buffer = &mut buf.buffer_mut()[offset..];

    match info.pixel_format {
        PixelFormat::Rgb => {
            if pixel_buffer.len() >= 3 {
                pixel_buffer[0] = color.red;
                pixel_buffer[1] = color.green;
                pixel_buffer[2] = color.blue;
            }
        }
        PixelFormat::Bgr => {
            if pixel_buffer.len() >= 3 {
                pixel_buffer[0] = color.blue;
                pixel_buffer[1] = color.green;
                pixel_buffer[2] = color.red;
            }
        }
        PixelFormat::U8 => {
            // simple grayscale average
            if pixel_buffer.len() >= 1 {
                let gray = color.red / 3 + color.green / 3 + color.blue / 3;
                pixel_buffer[0] = gray;
            }
        }
        other => panic!("unknown pixel format {other:?}"),
    }
}

pub struct Display<'f> {
    framebuffer: &'f mut FrameBuffer,
}

impl<'f> Display<'f> {
    pub fn new(framebuffer: &'f mut FrameBuffer) -> Display<'f> {
        Display { framebuffer }
    }

    fn draw_pixel(&mut self, Pixel(coordinates, color): Pixel<Rgb888>) {
        let info = self.framebuffer.info();
        let (width, height) = (info.width, info.height);

        let (x, y) = {
            let c: (i32, i32) = coordinates.into();
            (c.0 as usize, c.1 as usize)
        };

        if (0..width).contains(&x) && (0..height).contains(&y) {
            let color = Color { red: color.r(), green: color.g(), blue: color.b() };
            set_px_pos(self.framebuffer, Position { x, y }, color);
        }
    }
}

impl<'f> DrawTarget for Display<'f> {
    type Color = Rgb888;
    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for pixel in pixels {
            self.draw_pixel(pixel);
        }
        Ok(())
    }
}

impl<'f> OriginDimensions for Display<'f> {
    fn size(&self) -> Size {
        let info = self.framebuffer.info();
        Size::new(info.width as u32, info.height as u32)
    }
}

