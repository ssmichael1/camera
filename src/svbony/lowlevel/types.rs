#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub enum SVBBayerPattern {
    RG = 0,
    BG = 1,
    GR = 2,
    GB = 3,
}

impl From<i32> for SVBBayerPattern {
    fn from(value: i32) -> Self {
        match value {
            0 => SVBBayerPattern::RG,
            1 => SVBBayerPattern::BG,
            2 => SVBBayerPattern::GR,
            3 => SVBBayerPattern::GB,
            _ => panic!("Unknown SVBBayerPattern value: {}", value),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
#[repr(C)]
pub enum SVBImageType {
    SVBImageRaw8 = 0,
    SVBImageRaw10 = 2,
    SVBImageRaw12 = 3,
    SVBImageRaw14 = 4,
    SVBImageRaw16 = 5,
    SVBImageY8 = 6,
    SVBImageY10 = 7,
    SVBImageY12 = 8,
    SVBImageY14 = 9,
    SVBImageY16 = 10,
    SVBImageRGB24 = 11,
    SVBImageRGB32 = 12,
    SVBImageEnd = -1,
}

impl From<i32> for SVBImageType {
    fn from(value: i32) -> Self {
        match value {
            0 => SVBImageType::SVBImageRaw8,
            2 => SVBImageType::SVBImageRaw10,
            3 => SVBImageType::SVBImageRaw12,
            4 => SVBImageType::SVBImageRaw14,
            5 => SVBImageType::SVBImageRaw16,
            6 => SVBImageType::SVBImageY8,
            7 => SVBImageType::SVBImageY10,
            8 => SVBImageType::SVBImageY12,
            9 => SVBImageType::SVBImageY14,
            10 => SVBImageType::SVBImageY16,
            11 => SVBImageType::SVBImageRGB24,
            12 => SVBImageType::SVBImageRGB32,
            -1 => SVBImageType::SVBImageEnd,
            _ => panic!("Unknown SVBImageType value: {}", value),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub enum GuideDirection {
    North = 0,
    South = 1,
    East = 2,
    West = 3,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub enum FlipStatus {
    None = 0,
    Horizontal = 1,
    Vertical = 2,
    Both = 3,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SVBCameraMode {
    Normal = 0,
    TrigSoft = 1,
    TrigRiseEdge = 2,
    TrigFallEdge = 3,
    TrigDoubleEdge = 4,
    TrigHighLevel = 5,
    TrigLowLevel = 6,
}

impl From<i32> for SVBCameraMode {
    fn from(value: i32) -> Self {
        match value {
            0 => SVBCameraMode::Normal,
            1 => SVBCameraMode::TrigSoft,
            2 => SVBCameraMode::TrigRiseEdge,
            3 => SVBCameraMode::TrigFallEdge,
            4 => SVBCameraMode::TrigDoubleEdge,
            5 => SVBCameraMode::TrigHighLevel,
            6 => SVBCameraMode::TrigLowLevel,
            _ => panic!("Unknown SVBCameraMode value: {}", value),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
#[repr(i32)]
pub enum SVBErrorCode {
    Success = 0,
    InvalidIndex = 1,
    InvalidId = 2,
    InvalidControlType = 3,
    CameraClosed = 4,
    CameraRemoved = 5,
    InvalidPath = 6,
    InvalidFileFormat = 7,
    InvalidSize = 8,
    InvalidImageType = 9,
    OutOfBoundary = 10,
    Timeout = 11,
    InvalidSequence = 12,
    BufferTooSmall = 13,
    VideoModeActive = 14,
    ExposureInProgress = 15,
    GeneralError = 16,
    InvalidMode = 17,
    InvalidDirection = 18,
    UnknownSensorType = 19,
}

impl From<i32> for SVBErrorCode {
    fn from(value: i32) -> Self {
        match value {
            0 => SVBErrorCode::Success,
            1 => SVBErrorCode::InvalidIndex,
            2 => SVBErrorCode::InvalidId,
            3 => SVBErrorCode::InvalidControlType,
            4 => SVBErrorCode::CameraClosed,
            5 => SVBErrorCode::CameraRemoved,
            6 => SVBErrorCode::InvalidPath,
            7 => SVBErrorCode::InvalidFileFormat,
            8 => SVBErrorCode::InvalidSize,
            9 => SVBErrorCode::InvalidImageType,
            10 => SVBErrorCode::OutOfBoundary,
            11 => SVBErrorCode::Timeout,
            12 => SVBErrorCode::InvalidSequence,
            13 => SVBErrorCode::BufferTooSmall,
            14 => SVBErrorCode::VideoModeActive,
            15 => SVBErrorCode::ExposureInProgress,
            16 => SVBErrorCode::GeneralError,
            17 => SVBErrorCode::InvalidMode,
            18 => SVBErrorCode::InvalidDirection,
            19 => SVBErrorCode::UnknownSensorType,
            _ => panic!("Unknown error code: {}", value),
        }
    }
}

impl std::error::Error for SVBErrorCode {
    fn description(&self) -> &str {
        match self {
            SVBErrorCode::Success => "Success",
            SVBErrorCode::InvalidIndex => "Invalid index",
            SVBErrorCode::InvalidId => "Invalid ID",
            SVBErrorCode::InvalidControlType => "Invalid control type",
            SVBErrorCode::CameraClosed => "Camera closed",
            SVBErrorCode::CameraRemoved => "Camera removed",
            SVBErrorCode::InvalidPath => "Invalid path",
            SVBErrorCode::InvalidFileFormat => "Invalid file format",
            SVBErrorCode::InvalidSize => "Invalid size",
            SVBErrorCode::InvalidImageType => "Invalid image type",
            SVBErrorCode::OutOfBoundary => "Out of boundary",
            SVBErrorCode::Timeout => "Timeout",
            SVBErrorCode::InvalidSequence => "Invalid sequence",
            SVBErrorCode::BufferTooSmall => "Buffer too small",
            SVBErrorCode::VideoModeActive => "Video mode active",
            SVBErrorCode::ExposureInProgress => "Exposure in progress",
            SVBErrorCode::GeneralError => "General error",
            SVBErrorCode::InvalidMode => "Invalid mode",
            SVBErrorCode::InvalidDirection => "Invalid direction",
            SVBErrorCode::UnknownSensorType => "Unknown sensor type",
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }
}

impl std::fmt::Display for SVBErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SVBErrorCode::Success => write!(f, "Success"),
            SVBErrorCode::InvalidIndex => write!(f, "Invalid index"),
            SVBErrorCode::InvalidId => write!(f, "Invalid ID"),
            SVBErrorCode::InvalidControlType => write!(f, "Invalid control type"),
            SVBErrorCode::CameraClosed => write!(f, "Camera closed"),
            SVBErrorCode::CameraRemoved => write!(f, "Camera removed"),
            SVBErrorCode::InvalidPath => write!(f, "Invalid path"),
            SVBErrorCode::InvalidFileFormat => write!(f, "Invalid file format"),
            SVBErrorCode::InvalidSize => write!(f, "Invalid size"),
            SVBErrorCode::InvalidImageType => write!(f, "Invalid image type"),
            SVBErrorCode::OutOfBoundary => write!(f, "Out of boundary"),
            SVBErrorCode::Timeout => write!(f, "Timeout"),
            SVBErrorCode::InvalidSequence => write!(f, "Invalid sequence"),
            SVBErrorCode::BufferTooSmall => write!(f, "Buffer too small"),
            SVBErrorCode::VideoModeActive => write!(f, "Video mode active"),
            SVBErrorCode::ExposureInProgress => write!(f, "Exposure in progress"),
            SVBErrorCode::GeneralError => write!(f, "General error"),
            SVBErrorCode::InvalidMode => write!(f, "Invalid mode"),
            SVBErrorCode::InvalidDirection => write!(f, "Invalid direction"),
            SVBErrorCode::UnknownSensorType => write!(f, "Unknown sensor type"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub enum SVBBool {
    SVBFalse = 0,
    SVBTrue = 1,
}

impl From<SVBBool> for bool {
    fn from(value: SVBBool) -> Self {
        match value {
            SVBBool::SVBFalse => false,
            SVBBool::SVBTrue => true,
        }
    }
}

impl From<i32> for SVBBool {
    fn from(value: i32) -> Self {
        match value {
            0 => SVBBool::SVBFalse,
            1 => SVBBool::SVBTrue,
            _ => panic!("Unknown SVBBool value: {}", value),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SVBControlType {
    SVBGain = 0,
    SVBExposure = 1,
    SVBGamma = 2,
    SVBGammaContrast = 3,
    SVBWbRed = 4,
    SVBWbGreen = 5,
    SVBWbBlue = 6,
    SVBFlip = 7,
    SVBFrameSpeedMode = 8,
    SVBContrast = 9,
    SVBSharpness = 10,
    SVBSaturation = 11,
    SVBAutoTargetBrightness = 12,
    SVBBlackLevel = 13,
    SVBCoolerEnable = 14,
    SVBTargetTemperature = 15,
    SVBCurrentTemperature = 16,
    SVBCoolerPower = 17,
    SVBBadPixelCorrectionEnable = 18,
    SVBBadPixelCorrectionThreshold = 19,
}

impl From<i32> for SVBControlType {
    fn from(value: i32) -> Self {
        match value {
            0 => SVBControlType::SVBGain,
            1 => SVBControlType::SVBExposure,
            2 => SVBControlType::SVBGamma,
            3 => SVBControlType::SVBGammaContrast,
            4 => SVBControlType::SVBWbRed,
            5 => SVBControlType::SVBWbGreen,
            6 => SVBControlType::SVBWbBlue,
            7 => SVBControlType::SVBFlip,
            8 => SVBControlType::SVBFrameSpeedMode,
            9 => SVBControlType::SVBContrast,
            10 => SVBControlType::SVBSharpness,
            11 => SVBControlType::SVBSaturation,
            12 => SVBControlType::SVBAutoTargetBrightness,
            13 => SVBControlType::SVBBlackLevel,
            14 => SVBControlType::SVBCoolerEnable,
            15 => SVBControlType::SVBTargetTemperature,
            16 => SVBControlType::SVBCurrentTemperature,
            17 => SVBControlType::SVBCoolerPower,
            18 => SVBControlType::SVBBadPixelCorrectionEnable,
            19 => SVBControlType::SVBBadPixelCorrectionThreshold,
            _ => panic!("Unknown SVBControlType value: {}", value),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SVBExposureStatus {
    SVBExposureIdle = 0,
    SVBExposureInProgress = 1,
    SVBExposureSuccess = 2,
    SVBExposureFailed = 3,
}
