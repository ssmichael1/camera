use rand_distr::Distribution;

use crate::Camera;
use crate::CameraError;
use crate::CameraFrame;
use crate::FrameCallback;

use std::sync::{Arc, RwLock};
use std::thread;

pub struct SimCamera {
    exposure: f64,
    gain: f64,
    width: usize,
    height: usize,
    bit_depth: u8,
    frame_rate: f64,
    callback: Option<Arc<FrameCallback>>,
    running: bool,
    handle: Option<thread::JoinHandle<()>>,
}

impl SimCamera {
    pub fn new(width: usize, height: usize, bit_depth: u8) -> Self {
        Self {
            exposure: 0.0,
            gain: 0.0,
            width,
            height,
            bit_depth,
            frame_rate: 30.0,
            callback: None,
            running: false,
            handle: None,
        }
    }

    fn create_frame_data<T>(&self) -> Vec<u8>
    where
        T: crate::MonoPixel,
    {
        use rand_distr::Normal;

        let mut rng = rand::rng();
        let maxval = (1 << self.bit_depth as u32) - 1;
        let normal = Normal::new(0.0, ((maxval + 1) / 32) as f64).unwrap();
        let offset = (maxval + 1) / 8;
        let gval = ((maxval + 1) / 2) as f64;
        use std::f64::consts::PI;

        let now = chrono::Utc::now().timestamp_millis();
        let xoffset = (now as f64 * 2.0 * PI / 5000.0).cos() * 100.0;
        let yoffset = (now as f64 * 2.0 * PI / 3000.0 + PI / 4.0).cos() * 57.0;

        T::as_bytes(
            &(0..self.width * self.height)
                .map(|idx| {
                    let row = idx % self.width;
                    let col = idx / self.width;
                    let x = col as f64 - self.height as f64 / 2.0 - xoffset;
                    let y = row as f64 - self.width as f64 / 2.0 - yoffset;
                    let r = (x * x + y * y).sqrt();
                    let mut v = normal.sample(&mut rng) + offset as f64;
                    v += gval * f64::exp(-r * r / 100.0 / 100.0);
                    let v = v.round().clamp(0.0, maxval as f64) as i64;
                    num_traits::cast(v).unwrap()
                })
                .collect::<Vec<T>>(),
        )
        .to_vec()
    }

    fn create_frame(&self) -> CameraFrame {
        match self.bit_depth <= 8 {
            true => CameraFrame::new(
                self.exposure,
                chrono::Utc::now(),
                crate::PixelType::Gray8,
                Some(self.bit_depth),
                self.create_frame_data::<u8>(),
                self.width,
                self.height,
            ),
            false => CameraFrame::new(
                self.exposure,
                chrono::Utc::now(),
                crate::PixelType::Gray16,
                Some(self.bit_depth),
                self.create_frame_data::<u16>(),
                self.width,
                self.height,
            ),
        }
    }

    pub fn start(cam: Arc<RwLock<SimCamera>>) {
        let mut c = cam.write().unwrap();
        c.running = true;
        let cam = cam.clone();
        let handle = thread::spawn(move || loop {
            let sleeptime: u64;
            {
                let cam = cam.read().unwrap();
                if !cam.running {
                    break;
                }

                let frame = cam.create_frame();
                if let Some(ref callback) = cam.callback {
                    callback(frame).unwrap();
                }
                sleeptime = (1.0e6 / cam.frame_rate) as u64;
            }
            thread::sleep(std::time::Duration::from_micros(sleeptime));
        });
        c.handle = Some(handle);
    }
}

impl Camera for Arc<RwLock<SimCamera>> {
    fn connect(&mut self) -> Result<(), CameraError> {
        Ok(())
    }

    fn disconnect(&mut self) -> Result<(), CameraError> {
        Ok(())
    }

    fn set_exposure(&mut self, exposure: f64) -> Result<(), CameraError> {
        self.write().unwrap().exposure = exposure;
        Ok(())
    }

    fn get_exposure(&self) -> Result<f64, CameraError> {
        Ok(self.read().unwrap().exposure)
    }

    fn get_exposure_limits(&self) -> Result<(f64, f64), CameraError> {
        Ok((0.0, 0.0))
    }

    fn set_gain(&mut self, gain: f64) -> Result<(), CameraError> {
        self.write().unwrap().gain = gain;
        Ok(())
    }

    fn get_gain(&self) -> Result<f64, CameraError> {
        Ok(self.read().unwrap().gain)
    }

    fn get_roi(&self) -> Result<(u32, u32, u32, u32), CameraError> {
        Ok((
            0,
            0,
            self.read().unwrap().width as u32,
            self.read().unwrap().height as u32,
        ))
    }

    fn set_roi(&mut self, _x: u32, _y: u32, width: u32, height: u32) -> Result<(), CameraError> {
        self.write().unwrap().width = width as usize;
        self.write().unwrap().height = height as usize;
        Ok(())
    }

    fn get_max_roi(&self) -> Result<(u32, u32, u32, u32), CameraError> {
        Ok((0, 0, 0, 0))
    }

    fn name(&self) -> String {
        "Simulated Camera".to_string()
    }

    fn start(&mut self) -> Result<(), CameraError> {
        println!("starting sim camera");
        let cam = self.clone();
        SimCamera::start(cam);
        Ok(())
    }

    fn stop(&mut self) -> Result<(), CameraError> {
        let h = self.write().unwrap().handle.take();
        self.write().unwrap().running = false;
        if let Some(h) = h {
            h.join().unwrap();
        };
        Ok(())
    }

    fn set_frame_callback(&mut self, f: Box<FrameCallback>) -> Result<(), CameraError> {
        self.write().unwrap().callback = Some(Arc::new(f));
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sim() {
        println!("starting");
        let mut cam = Arc::new(RwLock::new(SimCamera::new(100, 100, 8)));
        println!("to connect");
        cam.set_exposure(0.1).unwrap();

        cam.set_frame_callback(Box::new(
            move |_t: CameraFrame| -> Result<(), CameraError> { Ok(()) },
        ))
        .unwrap();
        cam.start().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(400));
        cam.stop().unwrap();
    }
}
