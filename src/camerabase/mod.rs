use crate::CameraFrame;

use std::sync::{Arc, RwLock};

pub type FrameCallback =
    dyn Fn(&CameraFrame) -> Result<(), crate::CameraError> + Send + Sync + 'static;

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

pub trait CameraTrait {
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

    fn set_frame_callback<F>(&mut self, _cb: F) -> Result<(), CameraError>
    where
        F: Fn(&CameraFrame) -> Result<(), CameraError> + Send + Sync + 'static,
    {
        Err(CameraError::NotSupported)
    }

    fn name(&self) -> String;
}

#[derive(Clone)]
#[allow(clippy::large_enum_variant)]
pub enum Camera {
    #[cfg(feature = "svbony")]
    SVBony(Arc<RwLock<crate::svbony::SVBonyCamera>>),
    #[cfg(feature = "sim")]
    Sim(Arc<RwLock<crate::SimCamera>>),
}

impl CameraTrait for Camera {
    fn connect(&mut self) -> Result<(), CameraError> {
        match self {
            #[cfg(feature = "svbony")]
            Camera::SVBony(cam) => cam.write().unwrap().connect(),
            #[cfg(feature = "sim")]
            Camera::Sim(cam) => cam.connect(),
        }
    }

    fn disconnect(&mut self) -> Result<(), CameraError> {
        match self {
            #[cfg(feature = "svbony")]
            Camera::SVBony(cam) => cam.write().unwrap().disconnect(),
            #[cfg(feature = "sim")]
            Camera::Sim(cam) => cam.disconnect(),
        }
    }

    fn set_exposure(&mut self, exposure: f64) -> Result<(), CameraError> {
        match self {
            #[cfg(feature = "svbony")]
            Camera::SVBony(cam) => cam.write().unwrap().set_exposure(exposure),
            #[cfg(feature = "sim")]
            Camera::Sim(cam) => cam.set_exposure(exposure),
        }
    }

    fn get_exposure(&self) -> Result<f64, CameraError> {
        match self {
            #[cfg(feature = "svbony")]
            Camera::SVBony(cam) => cam.read().unwrap().get_exposure(),
            #[cfg(feature = "sim")]
            Camera::Sim(cam) => cam.get_exposure(),
        }
    }

    fn get_exposure_limits(&self) -> Result<(f64, f64), CameraError> {
        match self {
            #[cfg(feature = "svbony")]
            Camera::SVBony(cam) => cam.read().unwrap().get_exposure_limits(),
            #[cfg(feature = "sim")]
            Camera::Sim(cam) => cam.get_exposure_limits(),
        }
    }

    fn set_gain(&mut self, gain: f64) -> Result<(), CameraError> {
        match self {
            #[cfg(feature = "svbony")]
            Camera::SVBony(cam) => cam.write().unwrap().set_gain(gain),
            #[cfg(feature = "sim")]
            Camera::Sim(cam) => cam.set_gain(gain),
        }
    }

    fn get_gain(&self) -> Result<f64, CameraError> {
        match self {
            #[cfg(feature = "svbony")]
            Camera::SVBony(cam) => cam.read().unwrap().get_gain(),
            #[cfg(feature = "sim")]
            Camera::Sim(cam) => cam.get_gain(),
        }
    }

    fn get_roi(&self) -> Result<(u32, u32, u32, u32), CameraError> {
        match self {
            #[cfg(feature = "svbony")]
            Camera::SVBony(cam) => cam.read().unwrap().get_roi(),
            #[cfg(feature = "sim")]
            Camera::Sim(cam) => cam.get_roi(),
        }
    }

    fn set_roi(&mut self, x: u32, y: u32, width: u32, height: u32) -> Result<(), CameraError> {
        match self {
            #[cfg(feature = "svbony")]
            Camera::SVBony(cam) => cam.write().unwrap().set_roi(x, y, width, height),
            #[cfg(feature = "sim")]
            Camera::Sim(cam) => cam.set_roi(x, y, width, height),
        }
    }

    fn get_max_roi(&self) -> Result<(u32, u32, u32, u32), CameraError> {
        match self {
            #[cfg(feature = "svbony")]
            Camera::SVBony(cam) => cam.read().unwrap().get_max_roi(),
            #[cfg(feature = "sim")]
            Camera::Sim(cam) => cam.get_max_roi(),
        }
    }

    fn start(&mut self) -> Result<(), CameraError> {
        match self {
            #[cfg(feature = "svbony")]
            Camera::SVBony(cam) => cam.write().unwrap().start(),
            #[cfg(feature = "sim")]
            Camera::Sim(cam) => cam.start(),
        }
    }

    fn stop(&mut self) -> Result<(), CameraError> {
        match self {
            #[cfg(feature = "svbony")]
            Camera::SVBony(cam) => cam.write().unwrap().stop(),
            #[cfg(feature = "sim")]
            Camera::Sim(cam) => cam.stop(),
        }
    }

    fn set_frame_callback<F>(&mut self, cb: F) -> Result<(), CameraError>
    where
        F: Fn(&CameraFrame) -> Result<(), CameraError> + Send + Sync + 'static,
    {
        match self {
            #[cfg(feature = "svbony")]
            Camera::SVBony(cam) => cam.write().unwrap().set_frame_callback(cb),
            #[cfg(feature = "sim")]
            Camera::Sim(cam) => cam.set_frame_callback(cb),
        }
    }

    fn name(&self) -> String {
        match self {
            #[cfg(feature = "svbony")]
            Camera::SVBony(cam) => cam.read().unwrap().name(),
            #[cfg(feature = "sim")]
            Camera::Sim(cam) => cam.name(),
        }
    }
}
