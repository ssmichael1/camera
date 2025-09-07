use numeris::image::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CameraFrameError {
    #[error("Failed to save image")]
    SaveImageError,
    #[error("Unsupported image format")]
    UnsupportedFormatError,
}

impl From<CameraFrameError> for Result<(), CameraFrameError> {
    fn from(err: CameraFrameError) -> Self {
        Err(err)
    }
}

#[derive(Clone, Debug)]
pub enum FrameType {
    U8(Image<u8>),
    U16(Image<u16>),
    U32(Image<u32>),
    U64(Image<u64>),
    I8(Image<i8>),
    I16(Image<i16>),
    I32(Image<i32>),
    I64(Image<i64>),
    F32(Image<f32>),
    Rgb8(Image<RGB<u8>>),
    Rgba8(Image<RGBA<u8>>),
    Rgb16(Image<RGB<u16>>),
    Rgba16(Image<RGBA<u16>>),
}

impl<T> From<Image<T>> for FrameType
where
    T: PixelType + num_traits::PrimInt + num_traits::Unsigned,
{
    fn from(img: Image<T>) -> Self {
        match std::any::TypeId::of::<T>() {
            id if id == std::any::TypeId::of::<u8>() => FrameType::U8(unsafe {
                std::mem::transmute::<numeris::image::Image<T>, numeris::image::Image<u8>>(img)
            }),
            id if id == std::any::TypeId::of::<u16>() => FrameType::U16(unsafe {
                std::mem::transmute::<numeris::image::Image<T>, numeris::image::Image<u16>>(img)
            }),
            id if id == std::any::TypeId::of::<u32>() => FrameType::U32(unsafe {
                std::mem::transmute::<numeris::image::Image<T>, numeris::image::Image<u32>>(img)
            }),
            id if id == std::any::TypeId::of::<u64>() => FrameType::U64(unsafe {
                std::mem::transmute::<numeris::image::Image<T>, numeris::image::Image<u64>>(img)
            }),
            id if id == std::any::TypeId::of::<i8>() => FrameType::I8(unsafe {
                std::mem::transmute::<numeris::image::Image<T>, numeris::image::Image<i8>>(img)
            }),
            id if id == std::any::TypeId::of::<i16>() => FrameType::I16(unsafe {
                std::mem::transmute::<numeris::image::Image<T>, numeris::image::Image<i16>>(img)
            }),
            id if id == std::any::TypeId::of::<i32>() => FrameType::I32(unsafe {
                std::mem::transmute::<numeris::image::Image<T>, numeris::image::Image<i32>>(img)
            }),
            id if id == std::any::TypeId::of::<i64>() => FrameType::I64(unsafe {
                std::mem::transmute::<numeris::image::Image<T>, numeris::image::Image<i64>>(img)
            }),
            id if id == std::any::TypeId::of::<f32>() => FrameType::F32(unsafe {
                std::mem::transmute::<numeris::image::Image<T>, numeris::image::Image<f32>>(img)
            }),
            id if id == std::any::TypeId::of::<RGB<u8>>() => FrameType::Rgb8(unsafe {
                std::mem::transmute::<
                    numeris::image::Image<T>,
                    numeris::image::Image<numeris::image::RGB<u8>>,
                >(img)
            }),
            id if id == std::any::TypeId::of::<RGBA<u8>>() => FrameType::Rgba8(unsafe {
                std::mem::transmute::<
                    numeris::image::Image<T>,
                    numeris::image::Image<numeris::image::RGBA<u8>>,
                >(img)
            }),
            id if id == std::any::TypeId::of::<RGB<u16>>() => FrameType::Rgb16(unsafe {
                std::mem::transmute::<
                    numeris::image::Image<T>,
                    numeris::image::Image<numeris::image::RGB<u16>>,
                >(img)
            }),
            id if id == std::any::TypeId::of::<RGBA<u16>>() => FrameType::Rgba16(unsafe {
                std::mem::transmute::<
                    numeris::image::Image<T>,
                    numeris::image::Image<numeris::image::RGBA<u16>>,
                >(img)
            }),
            _ => panic!("Unsupported pixel type"),
        }
    }
}

#[derive(Clone)]
pub struct CameraFrame {
    pub exposure: f64,
    pub center_of_integration: chrono::DateTime<chrono::Utc>,
    pub bit_depth: Option<u8>,
    pub frame: FrameType,
}

impl CameraFrame {
    pub fn new<T>(
        exposure: f64,
        center_of_integration: chrono::DateTime<chrono::Utc>,
        bit_depth: Option<u8>,
        image: Image<T>,
    ) -> CameraFrame
    where
        T: PixelType + num_traits::PrimInt + num_traits::Unsigned,
    {
        CameraFrame {
            exposure,
            center_of_integration,
            bit_depth,
            frame: FrameType::from(image),
        }
    }
}

impl Default for CameraFrame {
    fn default() -> Self {
        CameraFrame {
            exposure: 0.0,
            center_of_integration: chrono::Utc::now(),
            bit_depth: None,
            frame: FrameType::from(Image::<u8>::default()),
        }
    }
}
