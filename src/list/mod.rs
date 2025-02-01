use crate::Camera;
use crate::SimCamera;

use std::sync::{Arc, RwLock};

pub struct AvailableCamera {
    pub name: String,
    pub get_camera: Box<dyn Fn() -> Box<dyn Camera>>,
}

pub fn get_available_cameras() -> Vec<AvailableCamera> {
    let mut v = vec![AvailableCamera {
        name: "Simulated Camera".to_string(),
        get_camera: Box::new(|| Box::new(Arc::new(RwLock::new(SimCamera::new(1920, 1080, 12))))),
    }];

    let sv = crate::svbony::get_connected_cameras().unwrap();
    sv.iter().for_each(|c| {
        v.push(AvailableCamera {
            name: c.friendly_name.clone(),
            get_camera: {
                let c = c.clone();
                Box::new(move || {
                    println!("camera= {:?}", c);
                    Box::new(crate::svbony::SVBonyCamera::new(0).unwrap())
                })
            },
        });
    });

    v
}
