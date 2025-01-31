use crate::Camera;
use crate::SimCamera;

use std::sync::{Arc, RwLock};

pub struct AvailableCamera {
    pub name: String,
    pub get_camera: fn() -> Box<dyn Camera>,
}

pub fn get_available_cameras() -> Vec<AvailableCamera> {
    vec![AvailableCamera {
        name: "Simulated Camera".to_string(),
        get_camera: || Box::new(Arc::new(RwLock::new(SimCamera::new(1920, 1080, 12)))),
    }]
}
