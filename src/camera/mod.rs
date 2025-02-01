use crate::CameraFrameType;

pub type FrameCallback = dyn Fn(CameraFrameType) -> Result<(), crate::CameraError> + Send + Sync;

#[derive(Debug, thiserror::Error)]
pub enum CameraError {
    #[error("Connection error")]
    Connection,
    #[error("Configuration error")]
    Config,
    #[error("Read error")]
    Read,
    #[error("Write error")]
    Write,
    #[error("Function not supported for this camera")]
    NotSupported,
    #[error("Error: {0}")]
    Other(String),
}

pub trait Camera {
    fn connect(&mut self) -> Result<(), CameraError> {
        Err(CameraError::NotSupported)
    }

    fn disconnect(&mut self) -> Result<(), CameraError> {
        Err(CameraError::NotSupported)
    }

    fn set_exposure(&mut self, _exposure: f64) -> Result<(), CameraError> {
        Err(CameraError::NotSupported)
    }

    fn get_exposure(&self) -> Result<f64, CameraError> {
        Err(CameraError::NotSupported)
    }

    fn get_exposure_limits(&self) -> Result<(f64, f64), CameraError> {
        Err(CameraError::NotSupported)
    }

    fn set_gain(&mut self, _gain: f64) -> Result<(), CameraError> {
        Err(CameraError::NotSupported)
    }

    fn get_gain(&self) -> Result<f64, CameraError> {
        Err(CameraError::NotSupported)
    }

    fn get_roi(&self) -> Result<(u32, u32, u32, u32), CameraError> {
        Err(CameraError::NotSupported)
    }

    fn set_roi(&mut self, _x: u32, _y: u32, _width: u32, _height: u32) -> Result<(), CameraError> {
        Err(CameraError::NotSupported)
    }

    fn get_max_roi(&self) -> Result<(u32, u32, u32, u32), CameraError> {
        Err(CameraError::NotSupported)
    }

    fn start(&mut self) -> Result<(), CameraError> {
        Err(CameraError::NotSupported)
    }

    fn stop(&mut self) -> Result<(), CameraError> {
        Err(CameraError::NotSupported)
    }

    fn set_frame_callback(&mut self, _f: Box<FrameCallback>) -> Result<(), CameraError> {
        Err(CameraError::NotSupported)
    }

    fn name(&self) -> String;
}
