use super::FrameData;

use crate::Pixel;

#[derive(Clone)]
pub struct CameraFrame<T>
where
    T: Pixel,
{
    pub exposure: f64,
    pub center_of_integration: chrono::DateTime<chrono::Utc>,
    pub bit_depth: u8,
    pub data: FrameData<T>,
}

impl<T> CameraFrame<T>
where
    T: Pixel,
{
    /// Create a new camera frame with given exposure, center of integration, bit depth, and raw data.
    ///
    /// # Arguments
    /// * `exposure` - The exposure time of the frame in seconds.
    /// * `center_of_integration` - The time at the center of the integration period.
    /// * `bit_depth` - The bit depth of the frame.
    /// * `raw` - The raw data of the frame.
    ///
    /// # Returns
    /// A new camera frame with the given exposure, center of integration, bit depth, and raw data.
    ///
    pub fn create(
        exposure: f64,
        center_of_integration: chrono::DateTime<chrono::Utc>,
        bit_depth: u8,
        raw: FrameData<T>,
    ) -> CameraFrame<T> {
        CameraFrame {
            exposure,
            center_of_integration,
            bit_depth,
            data: raw,
        }
    }
}

impl<T> Default for CameraFrame<T>
where
    T: Pixel,
{
    fn default() -> Self {
        CameraFrame {
            exposure: 0.0,
            center_of_integration: chrono::Utc::now(),
            bit_depth: 12,
            data: FrameData::<T>::default(),
        }
    }
}

impl<T> CameraFrame<T>
where
    T: Pixel,
{
    /// Get the width of the CameraFrame.
    /// The width is the number of columns in the CameraFrame.
    ///
    /// # Returns
    /// The width of the CameraFrame in pixels
    ///
    pub fn width(&self) -> usize {
        self.data.width as usize
    }

    /// Get the height of the CameraFrame.
    /// The height is the number of rows in the CameraFrame.
    ///
    /// # Returns
    /// The height of the CameraFrame in pixels
    pub fn height(&self) -> usize {
        self.data.height as usize
    }
}
