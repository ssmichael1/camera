/// Rgb crate says it will have this type, but not yet...
///

#[derive(Debug, Clone, Copy)]
pub enum PixelType {
    Gray8,
    Gray16,
    Rgb8,
    Rgb16,
    Rgba8,
    Rgba16,
}

pub trait MonoPixel: Pixel + num_traits::PrimInt {
    fn from_i64(value: i64) -> Self {
        num_traits::cast(value).unwrap()
    }

    fn mean(data: &[Self]) -> f64 {
        let total = data.iter().fold(0, |acc, x| acc + x.to_i64().unwrap());
        total as f64 / data.len() as f64
    }

    fn mean_and_var(data: &[Self]) -> (f64, f64) {
        let (sum, sumsq) = data.iter().fold((0, 0), |acc, x| {
            let (sum, sum_sq) = acc;
            let x: i64 = x.to_i64().unwrap();
            (sum + x, sum_sq + x * x)
        });
        let mean = sum as f64 / data.len() as f64;
        (mean, sumsq as f64 / data.len() as f64 - mean * mean)
    }

    fn sum(data: &[Self]) -> i64 {
        data.iter().fold(0, |acc, x| acc + x.to_i64().unwrap())
    }
}

impl MonoPixel for u8 {}
impl MonoPixel for u16 {}

pub trait Pixel:
    Sized
    + Clone
    + Copy
    + std::fmt::Debug
    + Send
    + Sync
    + 'static
    + Default
    + bytemuck::AnyBitPattern
    + bytemuck::NoUninit
{
    fn size(&self) -> usize;

    fn pixel_type(&self) -> PixelType;

    fn from_bytes(bytes: &[u8]) -> &[Self] {
        bytemuck::cast_slice(bytes)
    }

    fn as_bytes(pixels: &[Self]) -> &[u8] {
        bytemuck::cast_slice(pixels)
    }
}

impl Pixel for u8 {
    fn size(&self) -> usize {
        std::mem::size_of::<u8>()
    }

    fn pixel_type(&self) -> PixelType {
        PixelType::Gray8
    }
}

impl Pixel for u16 {
    fn size(&self) -> usize {
        std::mem::size_of::<u16>()
    }

    fn pixel_type(&self) -> PixelType {
        PixelType::Gray16
    }
}

impl Pixel for rgb::RGB8 {
    fn size(&self) -> usize {
        std::mem::size_of::<u8>() * 3
    }

    fn pixel_type(&self) -> PixelType {
        PixelType::Rgb8
    }
}

impl Pixel for rgb::RGB16 {
    fn size(&self) -> usize {
        std::mem::size_of::<u16>() * 3
    }

    fn pixel_type(&self) -> PixelType {
        PixelType::Rgb16
    }
}

impl Pixel for rgb::RGBA8 {
    fn size(&self) -> usize {
        std::mem::size_of::<u8>() * 4
    }

    fn pixel_type(&self) -> PixelType {
        PixelType::Rgba8
    }
}

impl Pixel for rgb::RGBA16 {
    fn size(&self) -> usize {
        std::mem::size_of::<u16>() * 4
    }

    fn pixel_type(&self) -> PixelType {
        PixelType::Rgba16
    }
}
