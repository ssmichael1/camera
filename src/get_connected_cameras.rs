use crate::Camera;

use std::sync::{Arc, RwLock};

#[cfg(feature = "svbony")]
use crate::svbony;

pub fn get_connected_cameras() -> Vec<Camera> {
    let mut cameras: Vec<Camera> = Vec::new();

    #[cfg(feature = "sim")]
    {
        cameras.push(Camera::Sim(Arc::new(RwLock::new(crate::SimCamera::new(
            640, 480, 8,
        )))))
    }
    #[cfg(feature = "svbony")]
    {
        let cams = crate::svbony::get_connected_cameras().unwrap();
        cameras.extend(cams.into_iter().enumerate().map(|(idx, _)| {
            Camera::SVBony(Arc::new(RwLock::new(
                svbony::SVBonyCamera::new(idx).unwrap(),
            )))
        }));
    }

    cameras
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::CameraTrait;

    #[test]
    fn test_get_connected_cameras() {
        let cams = get_connected_cameras();
        println!("found {} cameras", cams.len());
        for cam in cams {
            println!("camera: {}", cam.name());
        }
    }
}
