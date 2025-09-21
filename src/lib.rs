pub mod camerabase;
mod cameraframe;
pub mod colormap;
mod get_connected_cameras;
mod sim;

pub use cameraframe::*;

pub use get_connected_cameras::get_connected_cameras;
pub use sim::SimCamera;

pub use camerabase::{Camera, CameraError, CameraTrait, FrameCallback};

#[cfg(feature = "svbony")]
pub mod svbony;

pub mod prelude {
    pub use crate::camerabase::{Camera, CameraError, CameraTrait, FrameCallback};
    pub use crate::cameraframe::CameraFrame;
    pub use crate::colormap::*;
    pub use crate::get_connected_cameras::get_connected_cameras;
    #[cfg(feature = "sim")]
    pub use crate::sim::SimCamera;
    #[cfg(feature = "svbony")]
    pub use crate::svbony::SVBonyCamera;
}
