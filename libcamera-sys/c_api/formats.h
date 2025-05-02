#ifndef __LIBCAMERA_C_PIXEL_FORMAT_INFO_H__
#define __LIBCAMERA_C_PIXEL_FORMAT_INFO_H__

#include <stddef.h>
#include <stdint.h>
#include "pixel_format.h"
#include "geometry.h"

// Enum for ColourEncoding
enum libcamera_colour_encoding {
    LIBCAMERA_COLOUR_ENCODING_RGB,
    LIBCAMERA_COLOUR_ENCODING_YUV,
    LIBCAMERA_COLOUR_ENCODING_RAW,
};

// Struct for Plane
struct libcamera_pixel_format_info_plane {
    unsigned int bytes_per_group;
    unsigned int vertical_sub_sampling;
};

struct libcamera_pixel_format_info;

#ifdef __cplusplus
#include <libcamera/internal/formats.h>

typedef libcamera::PixelFormatInfo libcamera_pixel_format_info_t;

typedef libcamera::PixelFormatInfo::Plane libcamera_pixel_format_info_plane_t;
static_assert(sizeof(struct libcamera_pixel_format_info_plane) == sizeof(libcamera::PixelFormatInfo::Plane));
static_assert(offsetof(struct libcamera_pixel_format_info_plane, bytes_per_group) == offsetof(libcamera_pixel_format_info_plane_t, bytesPerGroup));
static_assert(offsetof(struct libcamera_pixel_format_info_plane, vertical_sub_sampling) == offsetof(libcamera_pixel_format_info_plane_t, verticalSubSampling));

extern "C" {
#else
typedef struct libcamera_pixel_format_info libcamera_pixel_format_info_t;
typedef struct libcamera_pixel_format_info_plane libcamera_pixel_format_info_plane_t;
#endif

// Check if the format info is valid
int libcamera_pixel_format_info_is_valid(const libcamera_pixel_format_info_t *info);

// Get pixel format info by various means
const libcamera_pixel_format_info_t *libcamera_pixel_format_info_get_by_format(const libcamera_pixel_format_t *format);
//const libcamera_pixel_format_info_t *libcamera_pixel_format_info_get_by_v4l2_format(const libcamera_v4l2_pixel_format_t *format);
const libcamera_pixel_format_info_t *libcamera_pixel_format_info_get_by_name(const char *name);

// Calculate various sizes
unsigned int libcamera_pixel_format_info_stride(const libcamera_pixel_format_info_t *info, 
                                               unsigned int width, 
                                               unsigned int plane,
                                               unsigned int align);

unsigned int libcamera_pixel_format_info_plane_size(const libcamera_pixel_format_info_t *info, 
                                                   const libcamera_size_t *size, 
                                                   unsigned int plane,
                                                   unsigned int align);

unsigned int libcamera_pixel_format_info_plane_size_with_stride(const libcamera_pixel_format_info_t *info, 
                                                                unsigned int height, 
                                                                unsigned int plane, 
                                                                unsigned int stride);

unsigned int libcamera_pixel_format_info_frame_size(const libcamera_pixel_format_info_t *info, 
                                                   const libcamera_size_t *size, 
                                                   unsigned int align);

unsigned int libcamera_pixel_format_info_frame_size_with_strides(const libcamera_pixel_format_info_t *info, 
                                                                const libcamera_size_t *size,
                                                                const unsigned int strides[3]);

// Get number of planes
unsigned int libcamera_pixel_format_info_num_planes(const libcamera_pixel_format_info_t *info);

// Field accessors
const char *libcamera_pixel_format_info_get_name(const libcamera_pixel_format_info_t *info);
libcamera_pixel_format_t libcamera_pixel_format_info_get_format(const libcamera_pixel_format_info_t *info);
unsigned int libcamera_pixel_format_info_get_bits_per_pixel(const libcamera_pixel_format_info_t *info);
enum libcamera_colour_encoding libcamera_pixel_format_info_get_colour_encoding(const libcamera_pixel_format_info_t *info);
int libcamera_pixel_format_info_is_packed(const libcamera_pixel_format_info_t *info);
unsigned int libcamera_pixel_format_info_get_pixels_per_group(const libcamera_pixel_format_info_t *info);
const struct libcamera_plane *libcamera_pixel_format_info_get_plane(const libcamera_pixel_format_info_t *info, unsigned int index);

#ifdef __cplusplus
}
#endif

#endif // __LIBCAMERA_C_PIXEL_FORMAT_INFO_H__
