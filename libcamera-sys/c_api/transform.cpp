#include <libcamera/transform.h>
#include "transform.h"
#include "orientation.h"

extern "C" {

libcamera_transform_t libcamera_transform_bitwise_and(libcamera_transform_t t1, libcamera_transform_t t2) {
    return static_cast<libcamera_transform_t>(
        static_cast<libcamera::Transform>(t1) & static_cast<libcamera::Transform>(t2)
    );
}

libcamera_transform_t libcamera_transform_bitwise_or(libcamera_transform_t t1, libcamera_transform_t t2) {
    return static_cast<libcamera_transform_t>(
        static_cast<libcamera::Transform>(t1) | static_cast<libcamera::Transform>(t2)
    );
}

libcamera_transform_t libcamera_transform_xor(libcamera_transform_t t1, libcamera_transform_t t2) {
    return static_cast<libcamera_transform_t>(
        static_cast<libcamera::Transform>(t1) ^ static_cast<libcamera::Transform>(t2)
    );
}

libcamera_transform_t libcamera_transform_mul(libcamera_transform_t t1, libcamera_transform_t t2) {
    return static_cast<libcamera_transform_t>(
        static_cast<libcamera::Transform>(t1) * static_cast<libcamera::Transform>(t2)
    );
}

libcamera_transform_t libcamera_transform_inv(libcamera_transform_t t) {
    return static_cast<libcamera_transform_t>(
        -static_cast<libcamera::Transform>(t)
    );
}

libcamera_transform_t libcamera_transform_bitwise_not(libcamera_transform_t t) {
    return static_cast<libcamera_transform_t>(
        ~static_cast<libcamera::Transform>(t)
    );
}

bool libcamera_transform_is_identity(libcamera_transform_t t) {
    return !static_cast<libcamera::Transform>(t);
}

libcamera_transform_t libcamera_transform_from_orientation(libcamera_orientation_t o) {
    return static_cast<libcamera_transform_t>(
        libcamera::transformFromOrientation(static_cast<libcamera::Orientation>(o))
    );
}

libcamera_transform_t libcamera_orientation_mul_transform(libcamera_orientation_t o, libcamera_transform_t t) {
    return static_cast<libcamera_transform_t>(
        static_cast<libcamera::Orientation>(o) * static_cast<libcamera::Transform>(t)
    );
}

libcamera_transform_t libcamera_orientations_divide(libcamera_orientation_t o1, libcamera_orientation_t o2) {
    return static_cast<libcamera_transform_t>(
        static_cast<libcamera::Orientation>(o1) / static_cast<libcamera::Orientation>(o2)
    );
}

libcamera_transform_t libcamera_transform_from_rotation(int angle, bool *success) {
    return static_cast<libcamera_transform_t>(
        libcamera::transformFromRotation(angle, success)
    );
}

}
