use libcamera_sys::*;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use thiserror::Error;

#[derive(Debug, Clone, Copy, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum Orientation {
    Rotate0 = libcamera_orientation::LIBCAMERA_ORIENTATION_ROTATE_0,
    Rotate0Mirror = libcamera_orientation::LIBCAMERA_ORIENTATION_ROTATE_0_MIRROR,
    Rotate180 = libcamera_orientation::LIBCAMERA_ORIENTATION_ROTATE_180,
    Rotate180Mirror = libcamera_orientation::LIBCAMERA_ORIENTATION_ROTATE_180_MIRROR,
    Rotate90Mirror = libcamera_orientation::LIBCAMERA_ORIENTATION_ROTATE_90_MIRROR,
    Rotate270 = libcamera_orientation::LIBCAMERA_ORIENTATION_ROTATE_270,
    Rotate270Mirror = libcamera_orientation::LIBCAMERA_ORIENTATION_ROTATE_270_MIRROR,
    Rotate90 = libcamera_orientation::LIBCAMERA_ORIENTATION_ROTATE_90,
}

#[derive(Error, Debug)]
pub enum OrientationError {
    #[error("Invalid orientation value")]
    ConversionError,
}

impl Orientation {
    pub fn try_from_rotation(angle: i32) -> Result<Self, ()> {
        let mut success = false;
        let orientation = unsafe { libcamera_orientation_from_rotation(angle, &mut success as *mut bool) };
        if success {
            Orientation::try_from_primitive(orientation as u32).or(Err(()))
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orientation_conversion() {
        assert_eq!(Orientation::try_from_rotation(0).unwrap(), Orientation::Rotate0);
        assert_eq!(Orientation::try_from_rotation(90).unwrap(), Orientation::Rotate90);
        assert_eq!(Orientation::try_from_rotation(180).unwrap(), Orientation::Rotate180);
        assert_eq!(Orientation::try_from_rotation(270).unwrap(), Orientation::Rotate270);
        assert!(Orientation::try_from_rotation(45).is_err());
    }
}
