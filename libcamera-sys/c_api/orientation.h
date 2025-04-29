#ifndef __LIBCAMERA_C_ORIENTATION__
#define __LIBCAMERA_C_ORIENTATION__

//#include <iostream>
//#include <stdint.h>
//#include <stdbool.h>
//#include <stddef.h>
//#include <cstddef>
//#include <sys/types.h>

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

enum libcamera_orientation {
    /* EXIF tag 274 starts from '1' */
    LIBCAMERA_ORIENTATION_ROTATE_0 = 1,
    LIBCAMERA_ORIENTATION_ROTATE_0_MIRROR,
    LIBCAMERA_ORIENTATION_ROTATE_180,
    LIBCAMERA_ORIENTATION_ROTATE_180_MIRROR,
    LIBCAMERA_ORIENTATION_ROTATE_90_MIRROR,
    LIBCAMERA_ORIENTATION_ROTATE_270,
    LIBCAMERA_ORIENTATION_ROTATE_270_MIRROR,
    LIBCAMERA_ORIENTATION_ROTATE_90,
};
typedef enum libcamera_orientation libcamera_orientation_t;

libcamera_orientation_t libcamera_orientation_from_rotation(int angle, bool *success);

#ifdef __cplusplus
}

#include <libcamera/orientation.h>
libcamera::Orientation libcamera_orientation_to_cpp_orientation(libcamera_orientation_t orientation);
libcamera_orientation_t libcamera_orientation_from_cpp_orientation(libcamera::Orientation orientation);
#endif // __cplusplus


#endif // __LIBCAMERA_C_ORIENTATION__
