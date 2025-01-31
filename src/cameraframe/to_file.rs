use super::FrameData;
use super::MonoFrameData;

use std::fs::File;

impl MonoFrameData<u8> {
    /// Save the FrameData to a PNG file.
    ///
    /// # Arguments
    /// `filename` - The name of the file to save the PNG to.
    ///
    /// # Returns
    /// An empty Result if the save was successful, or an error if the save failed.
    ///
    pub fn save_to_png(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut encoder = png::Encoder::new(File::create(filename)?, self.width, self.height);
        encoder.set_color(png::ColorType::Grayscale);
        let bitdepth = png::BitDepth::Eight;

        encoder.set_depth(bitdepth);
        let mut writer = encoder.write_header()?;
        writer.write_image_data(unsafe {
            std::slice::from_raw_parts(
                self.data
                    .iter()
                    .map(|x| x.0.swap_bytes())
                    .collect::<Vec<u8>>()
                    .as_ptr(),
                self.data.len(),
            )
        })?;
        Ok(())
    }
}

impl FrameData<rgb::RGBA8> {
    /// Save the FrameData to a PNG file.
    ///
    /// # Arguments
    /// `filename` - The name of the file to save the PNG to.
    ///
    /// # Returns
    /// An empty Result if the save was successful, or an error if the save failed.
    ///    
    pub fn save_to_png(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut encoder = png::Encoder::new(File::create(filename)?, self.width, self.height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;
        writer.write_image_data(unsafe {
            std::slice::from_raw_parts(
                self.data.as_ptr() as *const u8,
                self.data.len() * std::mem::size_of::<rgb::RGBA8>(),
            )
        })?;
        Ok(())
    }
}

impl FrameData<rgb::RGB8> {
    /// Save the FrameData to a PNG file.
    ///
    /// # Arguments
    /// `filename` - The name of the file to save the PNG to.
    ///
    /// # Returns
    /// An empty Result if the save was successful, or an error if the save failed.
    ///    
    pub fn save_to_png(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut encoder = png::Encoder::new(File::create(filename)?, self.width, self.height);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;
        writer.write_image_data(unsafe {
            std::slice::from_raw_parts(
                self.data.as_ptr() as *const u8,
                self.data.len() * std::mem::size_of::<rgb::RGB8>(),
            )
        })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> MonoFrameData<u16> {
        MonoFrameData::<u16> {
            width: 256,
            height: 256,
            data: (0..256 * 256)
                .map(|x| {
                    let mut row = (x % 256) as f64;
                    let mut col = (x / 256) as f64;
                    row -= 128.0;
                    row -= 10.0;
                    col -= 128.0;
                    col += 20.0;
                    rgb::Gray::<u16>::new(
                        (f64::exp(-(row * row + col * col) / 400.0) * 65535.0) as u16,
                    )
                })
                .collect(),
        }
    }

    #[test]
    fn test_save_to_png() {
        let data = test_data();
        let filename = "test.png";
        let _ = std::fs::remove_file(filename);
        let data2 = &data / 256;
        let data3: MonoFrameData<u8> = (&data2).into();
        let data4 = data3.to_rgba(0, 255, 1.0, crate::colormap::parula());
        data4.save_to_png(filename).unwrap();
        assert!(std::fs::metadata(filename).is_ok());
        //let _ = std::fs::remove_file(filename);
    }
}
