use super::CameraFrame;
use crate::PixelType;
use std::fs::File;

impl CameraFrame {
    pub fn save_to_png(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let (color_type, bit_depth) = match self.pixeltype {
            PixelType::Gray8 => (png::ColorType::Grayscale, png::BitDepth::Eight),
            PixelType::Gray16 => (png::ColorType::Grayscale, png::BitDepth::Sixteen),
            PixelType::Rgb8 => (png::ColorType::Rgb, png::BitDepth::Eight),
            PixelType::Rgb16 => (png::ColorType::Rgb, png::BitDepth::Sixteen),
            PixelType::Rgba8 => (png::ColorType::Rgba, png::BitDepth::Eight),
            PixelType::Rgba16 => (png::ColorType::Rgba, png::BitDepth::Sixteen),
        };
        let mut encoder = png::Encoder::new(
            File::create(filename)?,
            self.width as u32,
            self.height as u32,
        );
        encoder.set_color(color_type);
        encoder.set_depth(bit_depth);
        let mut writer = encoder.write_header()?;
        writer.write_image_data(&self.data)?;
        Ok(())
    }
}
