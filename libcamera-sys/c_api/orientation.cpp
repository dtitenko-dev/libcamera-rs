#include "orientation.h"
#include <libcamera/orientation.h>

extern "C" {

libcamera_orientation_t libcamera_orientation_from_rotation(int angle, bool *success) {
    return static_cast<libcamera_orientation_t>(libcamera::orientationFromRotation(angle, success));
}

}
