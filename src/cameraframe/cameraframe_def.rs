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

impl FrameType {
    pub fn width(&self) -> usize {
        match self {
            FrameType::U8(img) => img.width(),
            FrameType::U16(img) => img.width(),
            FrameType::U32(img) => img.width(),
            FrameType::U64(img) => img.width(),
            FrameType::I8(img) => img.width(),
            FrameType::I16(img) => img.width(),
            FrameType::I32(img) => img.width(),
            FrameType::I64(img) => img.width(),
            FrameType::F32(img) => img.width(),
            FrameType::Rgb8(img) => img.width(),
            FrameType::Rgba8(img) => img.width(),
            FrameType::Rgb16(img) => img.width(),
            FrameType::Rgba16(img) => img.width(),
        }
    }

    pub fn height(&self) -> usize {
        match self {
            FrameType::U8(img) => img.height(),
            FrameType::U16(img) => img.height(),
            FrameType::U32(img) => img.height(),
            FrameType::U64(img) => img.height(),
            FrameType::I8(img) => img.height(),
            FrameType::I16(img) => img.height(),
            FrameType::I32(img) => img.height(),
            FrameType::I64(img) => img.height(),
            FrameType::F32(img) => img.height(),
            FrameType::Rgb8(img) => img.height(),
            FrameType::Rgba8(img) => img.height(),
            FrameType::Rgb16(img) => img.height(),
            FrameType::Rgba16(img) => img.height(),
        }
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.width(), self.height())
    }

    pub fn size(&self) -> usize {
        self.width() * self.height()
    }

    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    pub fn bit_depth(&self) -> u8 {
        match self {
            FrameType::U8(_) | FrameType::I8(_) => 8,
            FrameType::U16(_) | FrameType::I16(_) => 16,
            FrameType::U32(_) | FrameType::I32(_) => 32,
            FrameType::U64(_) | FrameType::I64(_) => 64,
            FrameType::F32(_) => 32,
            FrameType::Rgb8(_) | FrameType::Rgba8(_) => 8,
            FrameType::Rgb16(_) | FrameType::Rgba16(_) => 16,
        }
    }

    pub fn channels(&self) -> u8 {
        match self {
            FrameType::U8(_)
            | FrameType::U16(_)
            | FrameType::U32(_)
            | FrameType::U64(_)
            | FrameType::I8(_)
            | FrameType::I16(_)
            | FrameType::I32(_)
            | FrameType::I64(_)
            | FrameType::F32(_) => 1,
            FrameType::Rgb8(_) | FrameType::Rgb16(_) => 3,
            FrameType::Rgba8(_) | FrameType::Rgba16(_) => 4,
        }
    }

    pub fn is_color(&self) -> bool {
        self.channels() > 1
    }

    pub fn is_grayscale(&self) -> bool {
        self.channels() == 1
    }

    pub fn is_floating_point(&self) -> bool {
        matches!(self, FrameType::F32(_))
    }

    pub fn is_integer(&self) -> bool {
        !self.is_floating_point()
    }

    pub fn is_signed(&self) -> bool {
        matches!(
            self,
            FrameType::I8(_) | FrameType::I16(_) | FrameType::I32(_) | FrameType::I64(_)
        )
    }

    pub fn raw<T>(&self) -> Option<&[T]>
    where
        T: PixelType,
    {
        match self {
            FrameType::U8(img) => {
                if std::any::TypeId::of::<T>() == std::any::TypeId::of::<u8>() {
                    Some(unsafe { std::mem::transmute::<&[u8], &[T]>(img.raw_slice()) })
                } else {
                    None
                }
            }
            FrameType::U16(img) => {
                if std::any::TypeId::of::<T>() == std::any::TypeId::of::<u16>() {
                    Some(unsafe { std::mem::transmute::<&[u16], &[T]>(img.raw_slice()) })
                } else {
                    None
                }
            }
            FrameType::U32(img) => {
                if std::any::TypeId::of::<T>() == std::any::TypeId::of::<u32>() {
                    Some(unsafe { std::mem::transmute::<&[u32], &[T]>(img.raw_slice()) })
                } else {
                    None
                }
            }
            FrameType::U64(img) => {
                if std::any::TypeId::of::<T>() == std::any::TypeId::of::<u64>() {
                    Some(unsafe { std::mem::transmute::<&[u64], &[T]>(img.raw_slice()) })
                } else {
                    None
                }
            }
            FrameType::I8(img) => {
                if std::any::TypeId::of::<T>() == std::any::TypeId::of::<i8>() {
                    Some(unsafe { std::mem::transmute::<&[i8], &[T]>(img.raw_slice()) })
                } else {
                    None
                }
            }
            FrameType::I16(img) => {
                if std::any::TypeId::of::<T>() == std::any::TypeId::of::<i16>() {
                    Some(unsafe { std::mem::transmute::<&[i16], &[T]>(img.raw_slice()) })
                } else {
                    None
                }
            }
            FrameType::I32(img) => {
                if std::any::TypeId::of::<T>() == std::any::TypeId::of::<i32>() {
                    Some(unsafe { std::mem::transmute::<&[i32], &[T]>(img.raw_slice()) })
                } else {
                    None
                }
            }
            FrameType::I64(img) => {
                if std::any::TypeId::of::<T>() == std::any::TypeId::of::<i64>() {
                    Some(unsafe { std::mem::transmute::<&[i64], &[T]>(img.raw_slice()) })
                } else {
                    None
                }
            }
            FrameType::F32(img) => {
                if std::any::TypeId::of::<T>() == std::any::TypeId::of::<f32>() {
                    Some(unsafe { std::mem::transmute::<&[f32], &[T]>(img.raw_slice()) })
                } else {
                    None
                }
            }
            FrameType::Rgb8(img) => {
                if std::any::TypeId::of::<T>() == std::any::TypeId::of::<RGB<u8>>() {
                    Some(unsafe { std::mem::transmute::<&[RGB<u8>], &[T]>(img.raw_slice()) })
                } else {
                    None
                }
            }
            FrameType::Rgba8(img) => {
                if std::any::TypeId::of::<T>() == std::any::TypeId::of::<RGBA<u8>>() {
                    Some(unsafe { std::mem::transmute::<&[RGBA<u8>], &[T]>(img.raw_slice()) })
                } else {
                    None
                }
            }
            FrameType::Rgb16(img) => {
                if std::any::TypeId::of::<T>() == std::any::TypeId::of::<RGB<u16>>() {
                    Some(unsafe { std::mem::transmute::<&[RGB<u16>], &[T]>(img.raw_slice()) })
                } else {
                    None
                }
            }
            FrameType::Rgba16(img) => {
                if std::any::TypeId::of::<T>() == std::any::TypeId::of::<RGBA<u16>>() {
                    Some(unsafe { std::mem::transmute::<&[RGBA<u16>], &[T]>(img.raw_slice()) })
                } else {
                    None
                }
            }
        }
    }
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
