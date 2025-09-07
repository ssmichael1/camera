use super::{CameraFrame, CameraFrameError, FrameType};
use numeris::image::PixelType;
use numeris::image::{RGB, RGBA};
use std::fs::File;

fn save_to_png_rgba<T>(
    img: &numeris::image::Image<RGBA<T>>,
    filename: &str,
) -> Result<(), CameraFrameError>
where
    T: PixelType + num_traits::PrimInt + num_traits::Unsigned,
{
    let (color_type, bit_depth) = if T::max_value() == T::from(255u8).unwrap() {
        (png::ColorType::Rgba, png::BitDepth::Eight)
    } else if T::max_value() == T::from(65535u16).unwrap() {
        (png::ColorType::Rgba, png::BitDepth::Sixteen)
    } else {
        return Err(CameraFrameError::SaveImageError);
    };

    let mut encoder = png::Encoder::new(
        File::create(filename).map_err(|_| CameraFrameError::SaveImageError)?,
        img.width() as u32,
        img.height() as u32,
    );
    encoder.set_color(color_type);
    encoder.set_depth(bit_depth);
    let mut writer = encoder
        .write_header()
        .map_err(|_| CameraFrameError::SaveImageError)?;

    writer
        .write_image_data(unsafe {
            let raw = img.raw();
            std::slice::from_raw_parts(
                raw.as_ptr() as *const u8,
                raw.len() * std::mem::size_of::<T>() * 4,
            )
        })
        .map_err(|_| CameraFrameError::SaveImageError)?;
    Ok(())
}

fn save_to_png_rgb<T>(
    img: &numeris::image::Image<RGB<T>>,
    filename: &str,
) -> Result<(), CameraFrameError>
where
    T: PixelType + num_traits::PrimInt + num_traits::Unsigned,
{
    let (color_type, bit_depth) = if T::max_value() == T::from(255u8).unwrap() {
        (png::ColorType::Rgb, png::BitDepth::Eight)
    } else if T::max_value() == T::from(65535u16).unwrap() {
        (png::ColorType::Rgb, png::BitDepth::Sixteen)
    } else {
        return Err(CameraFrameError::SaveImageError);
    };

    let mut encoder = png::Encoder::new(
        File::create(filename).map_err(|_| CameraFrameError::SaveImageError)?,
        img.width() as u32,
        img.height() as u32,
    );
    encoder.set_color(color_type);
    encoder.set_depth(bit_depth);
    let mut writer = encoder
        .write_header()
        .map_err(|_| CameraFrameError::SaveImageError)?;

    writer
        .write_image_data(unsafe {
            let raw = img.raw();
            std::slice::from_raw_parts(
                raw.as_ptr() as *const u8,
                raw.len() * std::mem::size_of::<T>() * 3,
            )
        })
        .map_err(|_| CameraFrameError::SaveImageError)?;
    Ok(())
}

fn save_to_png_mono<T>(
    img: &numeris::image::Image<T>,
    filename: &str,
) -> Result<(), CameraFrameError>
where
    T: PixelType + num_traits::PrimInt + num_traits::Unsigned,
{
    let (color_type, bit_depth) = if T::max_value() == T::from(255u8).unwrap() {
        (png::ColorType::Grayscale, png::BitDepth::Eight)
    } else if T::max_value() == T::from(65535u16).unwrap() {
        (png::ColorType::Grayscale, png::BitDepth::Sixteen)
    } else {
        return Err(CameraFrameError::SaveImageError);
    };

    let mut encoder = png::Encoder::new(
        File::create(filename).map_err(|_| CameraFrameError::SaveImageError)?,
        img.width() as u32,
        img.height() as u32,
    );
    encoder.set_color(color_type);
    encoder.set_depth(bit_depth);
    let mut writer = encoder
        .write_header()
        .map_err(|_| CameraFrameError::SaveImageError)?;

    writer
        .write_image_data(unsafe {
            let raw = img.raw();
            std::slice::from_raw_parts(raw.as_ptr() as *const u8, std::mem::size_of_val(raw))
        })
        .map_err(|_| CameraFrameError::SaveImageError)?;
    Ok(())
}

impl CameraFrame {
    pub fn save_to_ppm(&self, filename: &str) -> Result<(), CameraFrameError> {
        match self.frame {
            FrameType::Rgb8(ref img) => {
                let mut file =
                    File::create(filename).map_err(|_| CameraFrameError::SaveImageError)?;
                use std::io::Write;
                write!(file, "P6\n{} {}\n255\n", img.width(), img.height())
                    .map_err(|_| CameraFrameError::SaveImageError)?;
                file.write_all(unsafe {
                    let raw = img.raw();
                    std::slice::from_raw_parts(
                        raw.as_ptr() as *const u8,
                        raw.len() * std::mem::size_of::<u8>() * 3,
                    )
                })
                .map_err(|_| CameraFrameError::SaveImageError)?;
                Ok(())
            }
            _ => CameraFrameError::UnsupportedFormatError.into(),
        }
    }

    /// Saves the camera frame to a PNG file.
    ///
    /// # Arguments
    ///
    /// * `filename` - The name of the file to save the image to.
    ///
    /// # Example
    /// ```
    /// use camera::{CameraFrame, FrameType};
    /// use numeris::image::{Image, RGB};
    /// let img = Image::<RGB<u8>>::ones(100, 100);
    /// let frame = CameraFrame {
    ///     exposure: 0.1,
    ///     center_of_integration: chrono::Utc::now(),
    ///     bit_depth: Some(8),
    ///     frame: FrameType::Rgb8(img),
    /// };
    /// frame.save_to_png("output.png").unwrap();
    /// ```
    pub fn save_to_png(&self, filename: &str) -> Result<(), CameraFrameError> {
        match self.frame {
            FrameType::U8(ref img) => save_to_png_mono(img, filename),
            FrameType::U16(ref img) => save_to_png_mono(img, filename),
            FrameType::U32(ref img) => save_to_png_mono(img, filename),
            FrameType::U64(ref img) => save_to_png_mono(img, filename),
            FrameType::Rgb8(ref img) => save_to_png_rgb(img, filename),
            FrameType::Rgba8(ref img) => save_to_png_rgba(img, filename),
            FrameType::Rgb16(ref img) => save_to_png_rgb(img, filename),
            FrameType::Rgba16(ref img) => save_to_png_rgba(img, filename),
            _ => CameraFrameError::UnsupportedFormatError.into(),
        }
    }
}
