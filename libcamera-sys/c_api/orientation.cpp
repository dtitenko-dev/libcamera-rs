#include "orientation.h"
#include <libcamera/orientation.h>

libcamera::Orientation libcamera_orientation_to_cpp_orientation(libcamera_orientation_t orientation) {
    return static_cast<libcamera::Orientation>(orientation);
}
libcamera_orientation_t libcamera_orientation_from_cpp_orientation(libcamera::Orientation orientation) {
    return static_cast<libcamera_orientation_t>(orientation);
}

extern "C" {

libcamera_orientation_t libcamera_orientation_from_rotation(int angle, bool *success) {
    return libcamera_orientation_from_cpp_orientation(libcamera::orientationFromRotation(angle, success));
}

}
