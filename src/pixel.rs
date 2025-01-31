/// A trait for pixel types
pub trait Pixel: Sized + Clone + Copy + std::fmt::Debug + Send + Sync + 'static + Default {}

impl<T> Pixel for rgb::Gray<T> where
    T: num_traits::PrimInt + std::fmt::Debug + std::default::Default + Send + Sync + 'static
{
}
impl Pixel for rgb::RGBA8 {}
impl Pixel for rgb::RGBA16 {}
impl Pixel for rgb::RGB8 {}
impl Pixel for rgb::RGB16 {}

pub trait MonoPixel:
    num_traits::PrimInt + std::fmt::Debug + std::default::Default + Send + Sync + 'static
{
}
impl<T> MonoPixel for T where
    T: num_traits::PrimInt + std::fmt::Debug + std::default::Default + Send + Sync + 'static
{
}
