#ifndef __LIBCAMERA_C_TRANSFORM__
#define __LIBCAMERA_C_TRANSFORM__

#include <stdbool.h>
#include "orientation.h"

enum libcamera_transform {
    LIBCAMERA_TRANSFORM_IDENTITY = 0,
    LIBCAMERA_TRANSFORM_ROT_0 = LIBCAMERA_TRANSFORM_IDENTITY,
    LIBCAMERA_TRANSFORM_H_FLIP = 1,
    LIBCAMERA_TRANSFORM_V_FLIP = 2,
    LIBCAMERA_TRANSFORM_HV_FLIP = LIBCAMERA_TRANSFORM_H_FLIP | LIBCAMERA_TRANSFORM_V_FLIP,
    LIBCAMERA_TRANSFORM_TRANSPOSE = 4,
    LIBCAMERA_TRANSFORM_ROT_270 = LIBCAMERA_TRANSFORM_H_FLIP | LIBCAMERA_TRANSFORM_TRANSPOSE,
    LIBCAMERA_TRANSFORM_ROT_90 = LIBCAMERA_TRANSFORM_V_FLIP | LIBCAMERA_TRANSFORM_TRANSPOSE,
    LIBCAMERA_TRANSFORM_ROT_180_TRANSPOSE = LIBCAMERA_TRANSFORM_H_FLIP | LIBCAMERA_TRANSFORM_V_FLIP | LIBCAMERA_TRANSFORM_TRANSPOSE
};

typedef enum libcamera_transform libcamera_transform_t;



#ifdef __cplusplus
#include <libcamera/orientation.h>
#include <libcamera/transform.h>

namespace libcamera {
    extern libcamera::Transform transformFromOrientation(const Orientation &orientation);
}

extern "C" {
#endif // __cplusplus

libcamera_transform_t libcamera_transform_bitwise_and(libcamera_transform_t t1, libcamera_transform_t t2);
libcamera_transform_t libcamera_transform_bitwise_or(libcamera_transform_t t1, libcamera_transform_t t2);
libcamera_transform_t libcamera_transform_xor(libcamera_transform_t t1, libcamera_transform_t t2);
libcamera_transform_t libcamera_transform_mul(libcamera_transform_t t1, libcamera_transform_t t2);
libcamera_transform_t libcamera_transform_inv(libcamera_transform_t t);
libcamera_transform_t libcamera_transform_bitwise_not(libcamera_transform_t t);
bool libcamera_transform_is_identity(libcamera_transform_t t);

libcamera_transform_t libcamera_transform_from_orientation(libcamera_orientation_t o);

libcamera_transform_t libcamera_orientation_mul_transform(libcamera_orientation_t o, libcamera_transform_t t);
libcamera_transform_t libcamera_orientations_divide(libcamera_orientation_t o1, libcamera_orientation_t o2);


libcamera_transform_t libcamera_transform_from_rotation(int angle, bool *success);

#ifdef __cplusplus
}
#endif // __cplusplus


#endif
