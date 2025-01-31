mod cameraframe_def;
mod framedata;
mod mono_cast;
mod mono_ops;
mod mono_stats;
mod to_file;

pub use cameraframe_def::CameraFrame;
pub use framedata::FrameData;
pub use framedata::MonoFrameData;

pub type MonoCameraFrame<T> = CameraFrame<rgb::Gray<T>>;
pub type CameraFrameRGB = CameraFrame<rgb::RGB<u8>>;
pub type CameraFrameRGBA = CameraFrame<rgb::RGBA<u8>>;

pub enum CameraFrameType {
    Mono16(MonoCameraFrame<u16>),
    Mono8(MonoCameraFrame<u8>),
    RGB8(CameraFrameRGB),
    RGBA8(CameraFrameRGBA),
}
