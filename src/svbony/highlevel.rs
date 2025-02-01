use crate::svbony::lowlevel as ll;
use crate::Camera;
use crate::CameraFrame;
use crate::FrameCallback;
use crate::PixelType;

pub use ll::SVBCameraInfo;
pub use ll::{SVBBayerPattern, SVBCameraProperty, SVBErrorCode, SVBPixelType};
pub use ll::{SVBControlCaps, SVBControlType};

use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct SVBonyCamera {
    id: i32,
    info: SVBCameraInfo,
    property: SVBCameraProperty,
    pixel_pitch: f64,
    capabilities: Vec<SVBControlCaps>,
    running: Arc<Mutex<bool>>,
    callback: Option<Arc<FrameCallback>>,
}

type SVBResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Get a list of SVBony cameras connected to the host system
///
/// # Returns
///   A vector of SVBCameraInfo struct describing the connected cameras
pub fn get_connected_cameras() -> SVBResult<Vec<SVBCameraInfo>> {
    let ncam = ll::get_number_of_connected_camera();
    (0..ncam)
        .map(|i| match ll::get_camera_info(i) {
            Ok(info) => Ok(info),
            Err(e) => Err(e.into()),
        })
        .collect()
}

impl SVBonyCamera {
    /// Create a new camera object
    ///
    /// # Arguments
    /// * `num` - The camera number, index of the camera in the list of connected cameras from `get_connected_cameras`
    ///
    /// # Returns
    /// The camera object if successful
    /// Error if the camera cannot be opened
    ///
    pub fn new(num: usize) -> SVBResult<SVBonyCamera> {
        let info = ll::get_camera_info(num)?;
        let id: i32 = info.camera_id;
        ll::open_camera(id)?;

        Ok(SVBonyCamera {
            id: info.camera_id,
            info,
            property: ll::get_camera_property(&id)?,
            pixel_pitch: ll::get_pixel_size_microns(&id)? as f64,
            capabilities: {
                (0..ll::get_number_of_controls(&id)?)
                    .map(|i| -> SVBResult<SVBControlCaps> {
                        match ll::get_control_info(&id, i) {
                            Ok(info) => Ok(info),
                            Err(e) => Err(e.into()),
                        }
                    })
                    .collect::<SVBResult<Vec<SVBControlCaps>>>()?
            },
            running: Arc::new(Mutex::new(false)),
            callback: None,
        })
    }

    pub fn run(&mut self) -> SVBResult<()> {
        // Recommended timeout from vendor is 2x exposure time + 500ms
        // and exposure is queried in microseconds
        let wait_ms = self.get_exposure()? as i32 * 2 / 1000 + 500;
        *self.running.lock().unwrap() = true;
        self.set_control_value(SVBControlType::SVBFrameSpeedMode, 2)?;
        self.set_exposure(30000.0)?;
        ll::set_auto_save(&self.id, false)?;

        let max_pixels = (self.max_width() * self.max_height()) as usize;
        let mut buf8 = vec![0u8; max_pixels];
        let mut buf16 = vec![0u16; max_pixels];

        let pixeltype = ll::get_pixel_type(self.id)?;
        let exposure = self.get_exposure()?;
        let (_startx, _starty, width, height, bin) = ll::get_roi_format(self.id)?;
        let npixels = (width * height) as usize;
        let bit_depth = match pixeltype {
            SVBPixelType::Raw8 => 8 * bin * bin,
            SVBPixelType::Raw10 => 10 * bin * bin,
            SVBPixelType::Raw12 => 12 * bin * bin,
            SVBPixelType::Raw14 => 14 * bin * bin,
            SVBPixelType::Raw16 => 12 * bin * bin,
            _ => 8 * bin * bin,
        };
        println!("bit_depth = {}", bit_depth);
        println!("exposure = {}", exposure);
        println!("npixels = {}", npixels);
        println!("pixeltype = {:?}", pixeltype);

        ll::start_capture(&self.id)?;
        while *self.running.lock().unwrap() {
            match bit_depth {
                8 => {
                    let ts = self.get_frame(&mut buf8, wait_ms)?;
                    let frame = CameraFrame {
                        exposure,
                        center_of_integration: ts,
                        pixeltype: PixelType::Gray8,
                        bit_depth: Some(8),
                        data: buf8[..npixels].to_vec(),
                        width: width as usize,
                        height: height as usize,
                    };
                    if let Some(cb) = &self.callback {
                        cb(frame)?;
                    }
                }

                10 | 12 | 14 | 16 => {
                    let ts = self.get_frame(&mut buf16, wait_ms)?;
                    let frame = CameraFrame {
                        exposure,
                        center_of_integration: ts,
                        pixeltype: PixelType::Gray16,
                        bit_depth: Some(bit_depth as u8),
                        data: bytemuck::cast_slice(&buf16[..npixels]).to_vec(),
                        width: width as usize,
                        height: height as usize,
                    };
                    if let Some(cb) = &self.callback {
                        cb(frame)?;
                    }
                }
                _ => {
                    return Err("Unsupported bit depth".into());
                }
            }
        } // end of while loop
        ll::stop_capture(&self.id)
            .map_err(|e: SVBErrorCode| crate::CameraError::Other(e.to_string()))?;
        Ok(())
    }

    /// Close the camera
    ///
    /// # Notes:
    ///     The camera will be automatically closed when the object is dropped
    ///
    /// # Returns
    ///   Empty result if successful
    ///  Error if the camera cannot be closed
    ///
    pub fn close_camera(&mut self) -> SVBResult<()> {
        match ll::close_camera(&self.id) {
            Ok(_) => {
                self.id = -1;
                Ok(())
            }
            Err(e) => Err(e.into()),
        }
    }

    /// Get the info structure describign the camera
    ///
    /// # Returns
    ///    The camera info in SVBCameraInfo struct
    pub fn get_info(&self) -> &SVBCameraInfo {
        &self.info
    }

    /// Get number of dropped frames
    /// # Returns
    ///   The number of dropped frames
    pub fn dropped_frames(&self) -> i32 {
        ll::get_dropped_frames(&self.id).unwrap_or(0)
    }

    /// Get the camera pixel pitch in microns
    ///
    /// # Returns
    ///    The pixel pitch in microns
    pub fn pixel_pitch(&self) -> f64 {
        self.pixel_pitch
    }

    /// Set the camera exposure time in microseconds
    ///
    /// # Arguments
    ///     * `value` - The exposure time in microseconds
    ///
    /// # Returns
    ///    Empty result if successful
    ///   Error if the exposure time is invalid
    pub fn set_exposure(&self, value: i32) -> SVBResult<()> {
        self.set_control_value(SVBControlType::SVBExposure, value)
    }

    /// Get the camera gain
    ///
    /// # Returns
    ///     The gain value
    ///
    pub fn gain(&self) -> SVBResult<i32> {
        self.get_control_value(SVBControlType::SVBGain)
    }

    /// Set the camera gain
    /// # Arguments
    ///    * `value` - The gain value
    ///
    /// # Returns
    ///    Empty result if successful
    ///    Error if the gain value is invalid
    ///    
    pub fn set_gain(&self, value: i32) -> SVBResult<()> {
        self.set_control_value(SVBControlType::SVBGain, value)
    }

    /// Get the camera properties
    ///
    /// # Returns
    ///    The camera properties in SVBCameraProperty struct
    pub fn get_properties(&self) -> &ll::SVBCameraProperty {
        &self.property
    }

    /// Get maximum width (columns) in pixels of the camera
    ///
    /// # Returns
    ///   The maximum width in pixels
    pub fn max_width(&self) -> i32 {
        self.property.max_width
    }

    /// Get the maximum height (rows) in pixels of the camera
    ///
    /// # Returns
    ///   The maximum height in pixels
    pub fn max_height(&self) -> i32 {
        self.property.max_height
    }

    /// Query color or monochrome camera
    ///
    /// # Returns
    ///     True if the camera is color, false if monochrome
    pub fn is_color_cam(&self) -> bool {
        self.property.is_color_cam
    }

    /// Get the Bayer pattern of the camera
    ///
    /// # Notes
    ///      Only valid for color cameras
    ///
    /// # Returns
    ///     The Bayer pattern as SVBBayerPattern enum
    pub fn bayer_pattern(&self) -> SVBBayerPattern {
        self.property.bayer_pattern.clone()
    }

    /// Get supported pixel binning
    ///
    /// # Notes
    ///      Binning of pixels is a method of combining the charge from adjacent pixels
    ///
    /// # Returns
    ///    A vector of supported pixel binning values
    pub fn supported_bins(&self) -> Vec<i32> {
        self.property.supported_bins.clone()
    }

    /// Get supported video formats
    ///
    /// # Returns
    ///   A vector of supported video formats as SVBImageType enum
    pub fn supported_video_formats(&self) -> Vec<SVBPixelType> {
        self.property.supported_video_format.clone()
    }

    /// Get info for type of camera control
    ///
    /// # Arguments
    ///   * `ctrl` - The control type
    ///
    /// # Returns
    ///  The control info in SVBControlCaps struct
    pub fn get_control_info(&self, ctrl: SVBControlType) -> SVBResult<SVBControlCaps> {
        self.capabilities
            .iter()
            .find(|&x| x.control_type == ctrl)
            .cloned()
            .ok_or_else(|| "Control not found".into())
    }

    /// Set the value of a camera control
    ///
    /// # Arguments
    ///  * `ctrl` - The control type
    ///  * `value` - The value to set
    ///
    /// # Returns
    ///  Empty result if successful
    /// Error if the value is invalid
    ///
    pub fn set_control_value(&self, ctrl: SVBControlType, value: i32) -> SVBResult<()> {
        match ll::set_control_value(&self.id, ctrl, value, false) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    /// Query the value of a camera control
    ///
    /// # Arguments
    /// * `ctrl` - The control type
    ///
    /// # Returns
    /// The value of the control
    ///
    /// # Errors
    /// Error if the control is not found
    pub fn get_control_value(&self, ctrl: SVBControlType) -> SVBResult<i32> {
        match ll::get_control_value(&self.id, ctrl) {
            Ok(value) => Ok(value.0),
            Err(e) => Err(e.into()),
        }
    }

    pub fn get_frame<T>(
        &self,
        data: &mut [T],
        wait_ms: i32,
    ) -> Result<chrono::DateTime<chrono::Utc>, SVBErrorCode>
    where
        T: ll::PixelType,
    {
        match ll::get_video_data(&self.id, data, wait_ms) {
            Ok(ts) => Ok(ts),
            Err(e) => Err(e),
        }
    }
}

impl From<SVBErrorCode> for crate::CameraError {
    fn from(e: SVBErrorCode) -> crate::CameraError {
        crate::CameraError::Other(format!("{}", e))
    }
}

impl Camera for SVBonyCamera {
    fn name(&self) -> String {
        self.info.friendly_name.clone()
    }

    fn connect(&mut self) -> Result<(), crate::CameraError> {
        match ll::open_camera(self.id) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    fn disconnect(&mut self) -> Result<(), crate::CameraError> {
        match ll::close_camera(&self.id) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    fn start(&mut self) -> Result<(), crate::CameraError> {
        let mut cam = self.clone();
        std::thread::spawn(move || -> SVBResult<()> { cam.run() });
        Ok(())
    }

    fn get_exposure(&self) -> Result<f64, crate::CameraError> {
        ll::get_control_value(&self.id, SVBControlType::SVBExposure)
            .map(|v| v.0 as f64)
            .map_err(|e| e.into())
    }

    fn set_exposure(&mut self, exposure: f64) -> Result<(), crate::CameraError> {
        ll::set_control_value(
            &self.id,
            SVBControlType::SVBExposure,
            exposure as i32,
            false,
        )
        .map_err(|e| e.into())
    }

    fn get_max_roi(&self) -> Result<(u32, u32, u32, u32), crate::CameraError> {
        let prop = self.get_properties();
        Ok((0, 0, prop.max_width as u32, prop.max_height as u32))
    }

    fn stop(&mut self) -> Result<(), crate::CameraError> {
        *self.running.lock().unwrap() = false;
        Ok(())
    }

    fn set_frame_callback(&mut self, cb: Box<FrameCallback>) -> Result<(), crate::CameraError> {
        self.callback = Some(Arc::new(cb));
        Ok(())
    }
}

impl Drop for SVBonyCamera {
    fn drop(&mut self) {
        if self.id != -1 {
            let _ = ll::close_camera(&self.id);
        }
    }
}

impl std::fmt::Display for SVBonyCamera {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "SVBonyCamera")?;
        writeln!(f, "    Name: {}", self.info.friendly_name)?;
        writeln!(f, "    Serial Number: {}", self.info.serial_number)?;
        writeln!(f, "    Port Type: {}", self.info.port_type)?;
        writeln!(
            f,
            "    Format: {} x {}",
            self.max_width(),
            self.max_height()
        )?;
        writeln!(f, "    Color: {}", self.is_color_cam())?;
        if self.is_color_cam() {
            writeln!(f, "    Bayer Pattern: {:?}", self.bayer_pattern())?;
        }
        writeln!(f, "    Supported Bins: {:?}", self.supported_bins())?;
        writeln!(
            f,
            "    Supported Video Formats: {:?}",
            self.supported_video_formats()
        )?;
        writeln!(f, "    Pixel Pitch: {:.2} µm", self.pixel_pitch)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::CameraError;

    #[test]
    fn test_get_connected_cameras() {
        let cameras = get_connected_cameras().unwrap();
        println!("cameras = {:?}", cameras);
    }

    #[test]
    fn test_video_capture() {
        let cameras = get_connected_cameras().unwrap();
        if cameras.is_empty() {
            return;
        }
        let mut cam = SVBonyCamera::new(0).unwrap();
        cam.start().unwrap();
        let mut data = vec![0u8; (cam.max_width() * cam.max_height()) as usize];
        let ts = cam.get_frame(&mut data, 1000).unwrap();
        println!("ts = {}", ts);
        cam.stop().unwrap();
    }

    #[test]
    fn test_capabilities() {
        let cameras = get_connected_cameras().unwrap();
        if cameras.is_empty() {
            return;
        }
        let cam = SVBonyCamera::new(0).unwrap();
        cam.capabilities.iter().for_each(|c| {
            println!("{}", c);
        });
    }

    #[test]
    fn test_in_thread() {
        let cameras = get_connected_cameras().unwrap();
        if cameras.is_empty() {
            return;
        }
        let mut cam = SVBonyCamera::new(0).unwrap();
        cam.set_exposure(100000).unwrap();
        cam.set_frame_callback(Box::new(move |_f| -> Result<(), CameraError> {
            println!("ts");
            Ok(())
        }))
        .unwrap();
        let mut camclone = cam.clone();
        let handle = std::thread::spawn(move || -> SVBResult<()> { camclone.run() });
        std::thread::sleep(std::time::Duration::from_secs(1));
        cam.stop().unwrap();
        handle.join().unwrap().unwrap();
    }

    #[test]
    fn test_camera() {
        let cameras = get_connected_cameras().unwrap();
        if cameras.is_empty() {
            return;
        }
        let cam = SVBonyCamera::new(0).unwrap();

        cam.set_gain(30).unwrap();

        println!("cam = {}", cam);
        println!("exposure = {}", cam.get_exposure().unwrap());
        println!("gain = {}", cam.gain().unwrap());
    }
}
