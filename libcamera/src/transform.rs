use std::ops::{BitAnd, BitOr, BitXor, Div, Mul, Not};

use libcamera_sys::*;
use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::orientation::Orientation;

#[derive(Debug, Clone, Copy, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum Transform {
    Identity = libcamera_transform::LIBCAMERA_TRANSFORM_IDENTITY,
    HFlip = libcamera_transform::LIBCAMERA_TRANSFORM_H_FLIP,
    VFlip = libcamera_transform::LIBCAMERA_TRANSFORM_V_FLIP,
    Rot180 = libcamera_transform::LIBCAMERA_TRANSFORM_ROT_180,
    Transpose = libcamera_transform::LIBCAMERA_TRANSFORM_TRANSPOSE,
    Rot270 = libcamera_transform::LIBCAMERA_TRANSFORM_ROT_270,
    Rot90 = libcamera_transform::LIBCAMERA_TRANSFORM_ROT_90,
    Rot180Transpose = libcamera_transform::LIBCAMERA_TRANSFORM_ROT_180_TRANSPOSE,
}

impl Transform {
    pub fn is_identity(self) -> bool {
        unsafe { libcamera_transform_is_identity(self.into()) }
    }

    pub fn invert(self) -> Self {
        unsafe { Transform::try_from_primitive(libcamera_transform_inv(self.into())).unwrap() }
    }

    pub fn bitwise_not(self) -> Self {
        unsafe { Transform::try_from_primitive(libcamera_transform_bitwise_not(self.into())).unwrap() }
    }

    pub fn try_from_rotation(value: i32) -> Result<Self, ()> {
        let mut success = false;
        let result = unsafe { libcamera_transform_from_rotation(value, &mut success as *mut _) };
        if success {
            TryFromPrimitive::try_from_primitive(result).map_err(|_e| ())
        } else {
            Err(())
        }
    }
}

impl From<Orientation> for Transform {
    fn from(value: Orientation) -> Self {
        match value {
            Orientation::Rotate0 => Transform::Identity,
            Orientation::Rotate0Mirror => Transform::HFlip,
            Orientation::Rotate180 => Transform::Rot180,
            Orientation::Rotate180Mirror => Transform::VFlip,
            Orientation::Rotate90Mirror => Transform::Transpose,
            Orientation::Rotate90 => Transform::Rot90,
            Orientation::Rotate270Mirror => Transform::Rot180Transpose,
            Orientation::Rotate270 => Transform::Rot270,
        }
    }
}

impl Not for Transform {
    type Output = Transform;

    fn not(self) -> Self::Output {
        self.invert()
    }
}

impl BitAnd for Transform {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        unsafe { Transform::try_from_primitive(libcamera_transform_bitwise_and(self.into(), rhs.into())).unwrap() }
    }
}

impl BitOr for Transform {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe { Transform::try_from_primitive(libcamera_transform_bitwise_or(self.into(), rhs.into())).unwrap() }
    }
}

impl BitXor for Transform {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        unsafe { Transform::try_from_primitive(libcamera_transform_xor(self.into(), rhs.into())).unwrap() }
    }
}

impl Mul for Transform {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        unsafe { Transform::try_from_primitive(libcamera_transform_mul(self.into(), rhs.into())).unwrap() }
    }
}

impl Mul<Orientation> for Transform {
    type Output = Orientation;

    fn mul(self, rhs: Orientation) -> Self::Output {
        rhs * self
    }
}

impl Mul<Transform> for Orientation {
    type Output = Self;

    fn mul(self, rhs: Transform) -> Self::Output {
        unsafe {
            Orientation::try_from_primitive(libcamera_orientation_mul_transform(self.into(), rhs.into())).unwrap()
        }
    }
}

impl Div for Orientation {
    type Output = Transform;

    fn div(self, rhs: Self) -> Self::Output {
        unsafe { Transform::try_from_primitive(libcamera_orientations_divide(self.into(), rhs.into())).unwrap() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitwise_not() {
        assert_eq!(Transform::Identity.bitwise_not(), Transform::Rot180Transpose);
        assert_eq!(Transform::HFlip.bitwise_not(), Transform::Rot90);
        assert_eq!(Transform::VFlip.bitwise_not(), Transform::Rot270);
    }

    #[test]
    fn test_bitwise_and() {
        assert_eq!(Transform::HFlip & Transform::VFlip, Transform::Identity);
        assert_eq!(Transform::Identity & Transform::HFlip, Transform::Identity);
        assert_eq!(Transform::HFlip & Transform::HFlip, Transform::HFlip);
        assert_eq!(Transform::VFlip & Transform::VFlip, Transform::VFlip);
        assert_eq!(Transform::Rot180 & Transform::Rot180, Transform::Rot180);
        assert_eq!(Transform::HFlip & Transform::Rot180, Transform::HFlip);
        assert_eq!(Transform::VFlip & Transform::Rot180, Transform::VFlip);
        assert_eq!(Transform::Rot180Transpose & Transform::Rot180, Transform::Rot180);
    }

    #[test]
    fn test_bitwise_or() {
        assert_eq!(Transform::HFlip | Transform::VFlip, Transform::Rot180);
        assert_eq!(Transform::Identity | Transform::HFlip, Transform::HFlip);
        assert_eq!(Transform::HFlip | Transform::HFlip, Transform::HFlip);
        assert_eq!(Transform::VFlip | Transform::VFlip, Transform::VFlip);
        assert_eq!(
            Transform::Rot270 | Transform::Rot180Transpose,
            Transform::Rot180Transpose
        );
    }

    #[test]
    fn test_xor() {
        assert_eq!(Transform::HFlip ^ Transform::VFlip, Transform::Rot180);
        assert_eq!(Transform::Identity ^ Transform::HFlip, Transform::HFlip);
        assert_eq!(Transform::HFlip ^ Transform::HFlip, Transform::Identity);
        assert_eq!(Transform::VFlip ^ Transform::Identity, Transform::VFlip);
    }

    #[test]
    fn test_invert() {
        assert_eq!(Transform::Identity.invert(), Transform::Identity);
        assert_eq!(Transform::Rot180.invert(), Transform::Rot180);
        assert_eq!(Transform::Rot180Transpose.invert(), Transform::Rot180Transpose);

        assert_eq!(Transform::Rot270.invert(), Transform::Rot90);
        assert_eq!(Transform::Rot90.invert(), Transform::Rot270);
    }

    #[test]
    fn test_from_rotation() {
        assert_eq!(Transform::try_from_rotation(0), Ok(Transform::Identity));
        assert_eq!(Transform::try_from_rotation(90), Ok(Transform::Rot90));
        assert_eq!(Transform::try_from_rotation(-90), Ok(Transform::Rot270));
        assert_eq!(Transform::try_from_rotation(180), Ok(Transform::Rot180));
        assert_eq!(Transform::try_from_rotation(-180), Ok(Transform::Rot180));
        assert_eq!(Transform::try_from_rotation(270), Ok(Transform::Rot270));
        assert_eq!(Transform::try_from_rotation(-270), Ok(Transform::Rot90));
        assert_eq!(Transform::try_from_rotation(0), Ok(Transform::Identity));

        assert_eq!(Transform::try_from_rotation(1), Err(()));
        assert_eq!(Transform::try_from_rotation(9), Err(()));
        assert_eq!(Transform::try_from_rotation(18), Err(()));
        assert_eq!(Transform::try_from_rotation(45), Err(()));
        assert_eq!(Transform::try_from_rotation(89), Err(()));

        assert_eq!(Transform::try_from_rotation(135), Err(()));

        assert_eq!(Transform::try_from_rotation(-11), Err(()));
        assert_eq!(Transform::try_from_rotation(-9), Err(()));
        assert_eq!(Transform::try_from_rotation(-181), Err(()));
        assert_eq!(Transform::try_from_rotation(-45), Err(()));
        assert_eq!(Transform::try_from_rotation(-89), Err(()));
    }

    #[test]
    fn test_multiply() {
        let mut transformation = Transform::Identity;
        transformation = transformation * Transform::VFlip;
        assert_eq!(transformation, Transform::VFlip);
        transformation = transformation * Transform::HFlip;
        assert_eq!(transformation, Transform::Rot180);
        transformation = transformation * Transform::Rot90;
        assert_eq!(transformation, Transform::Rot270);
        transformation = transformation * Transform::Rot270;
        assert_eq!(transformation, Transform::Rot180);
        transformation = transformation * Transform::Rot180;
        assert_eq!(transformation, Transform::Identity);
        transformation = transformation * Transform::Transpose;
        assert_eq!(transformation, Transform::Transpose);
        transformation = transformation * Transform::Rot90;
        assert_eq!(transformation, Transform::HFlip);
        transformation = transformation * Transform::Rot90;
        assert_eq!(transformation, Transform::Rot180Transpose);
        transformation = transformation * Transform::Rot90;
        assert_eq!(transformation, Transform::VFlip);
        transformation = transformation * Transform::Identity;
        assert_eq!(transformation, Transform::VFlip);
        transformation = transformation * Transform::Transpose;
        assert_eq!(transformation, Transform::Rot90);
        transformation = transformation * !Transform::Rot90;
        assert_eq!(transformation, Transform::Identity);
    }

    #[test]
    fn test_divide_orientations() {
        assert_eq!(Orientation::Rotate0 / Orientation::Rotate0, Transform::Identity);
        assert_eq!(Orientation::Rotate0 / Orientation::Rotate90, Transform::Rot270);
        assert_eq!(Orientation::Rotate0 / Orientation::Rotate180, Transform::Rot180);
        assert_eq!(Orientation::Rotate0 / Orientation::Rotate270, Transform::Rot90);
        assert_eq!(Orientation::Rotate90 / Orientation::Rotate0, Transform::Rot90);
        assert_eq!(Orientation::Rotate90 / Orientation::Rotate90, Transform::Identity);
        assert_eq!(Orientation::Rotate90 / Orientation::Rotate180, Transform::Rot270);
        assert_eq!(Orientation::Rotate90 / Orientation::Rotate270, Transform::Rot180);
        assert_eq!(Orientation::Rotate180 / Orientation::Rotate0, Transform::Rot180);
        assert_eq!(
            Orientation::Rotate0Mirror / Orientation::Rotate90Mirror,
            Transform::Rot90
        );
    }

    #[test]
    fn test_mul_orientation_and_transform() {
        assert_eq!(Orientation::Rotate0 * Transform::Rot90, Orientation::Rotate90);
        assert_eq!(Orientation::Rotate90 * Transform::Rot90, Orientation::Rotate180);
        assert_eq!(Orientation::Rotate180 * Transform::Rot90, Orientation::Rotate270);
        assert_eq!(Orientation::Rotate270 * Transform::Rot90, Orientation::Rotate0);
        assert_eq!(Orientation::Rotate0 * Transform::Rot180, Orientation::Rotate180);
        assert_eq!(
            Orientation::Rotate90 * Transform::Rot180 * Transform::VFlip,
            Orientation::Rotate90Mirror
        );
        assert_eq!(Orientation::Rotate180 * Transform::Rot180, Orientation::Rotate0);
        assert_eq!(Orientation::Rotate270 * Transform::Rot180, Orientation::Rotate90);
        assert_eq!(
            Orientation::Rotate90Mirror * Transform::Rot270,
            Orientation::Rotate180Mirror
        );
    }

    #[test]
    fn test_from_orientation() {
        assert_eq!(Transform::from(Orientation::Rotate0), Transform::Identity);
        assert_eq!(Transform::from(Orientation::Rotate90), Transform::Rot90);
        assert_eq!(Transform::from(Orientation::Rotate180), Transform::Rot180);
        assert_eq!(Transform::from(Orientation::Rotate270), Transform::Rot270);
        assert_eq!(Transform::from(Orientation::Rotate0Mirror), Transform::HFlip);
        assert_eq!(Transform::from(Orientation::Rotate90Mirror), Transform::Transpose);
        assert_eq!(Transform::from(Orientation::Rotate180Mirror), Transform::VFlip);
        assert_eq!(
            Transform::from(Orientation::Rotate270Mirror),
            Transform::Rot180Transpose
        );
    }
}
