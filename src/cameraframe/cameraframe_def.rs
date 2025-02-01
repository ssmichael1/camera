use crate::PixelType;

#[derive(Clone)]
pub struct CameraFrame {
    pub exposure: f64,
    pub center_of_integration: chrono::DateTime<chrono::Utc>,
    pub pixeltype: PixelType,
    pub bit_depth: Option<u8>,
    pub data: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl CameraFrame {
    pub fn new(
        exposure: f64,
        center_of_integration: chrono::DateTime<chrono::Utc>,
        pixeltype: PixelType,
        bit_depth: Option<u8>,
        data: Vec<u8>,
        width: usize,
        height: usize,
    ) -> CameraFrame {
        CameraFrame {
            exposure,
            center_of_integration,
            pixeltype,
            bit_depth,
            data,
            width,
            height,
        }
    }
}

impl Default for CameraFrame {
    fn default() -> Self {
        CameraFrame {
            exposure: 0.0,
            center_of_integration: chrono::Utc::now(),
            pixeltype: PixelType::Gray8,
            bit_depth: None,
            data: Vec::new(),
            width: 0,
            height: 0,
        }
    }
}
