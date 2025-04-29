use std::{
    cmp::Ordering,
    ops::{Add, Div, Mul, Sub},
};

use libcamera_sys::*;

/// Represents `libcamera::Point`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl From<libcamera_point_t> for Point {
    fn from(p: libcamera_point_t) -> Self {
        Self { x: p.x, y: p.y }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x.saturating_add(rhs.x),
            y: self.y.saturating_add(rhs.y),
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x.saturating_sub(rhs.x),
            y: self.y.saturating_sub(rhs.y),
        }
    }
}

/// Represents `libcamera::Size`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Mul<f32> for Size {
    type Output = Size;

    fn mul(self, factor: f32) -> Self::Output {
        Size {
            width: (self.width as f32 * factor) as u32,
            height: (self.height as f32 * factor) as u32,
        }
    }
}

impl Div<f32> for Size {
    type Output = Size;

    fn div(self, factor: f32) -> Self::Output {
        Size {
            width: (self.width as f32 / factor) as u32,
            height: (self.height as f32 / factor) as u32,
        }
    }
}
impl Add for Size {
    type Output = Size;

    fn add(self, margins: Size) -> Self::Output {
        self.grown_by(&margins)
    }
}

impl Sub for Size {
    type Output = Size;

    fn sub(self, margins: Size) -> Self::Output {
        self.shrunk_by(&margins)
    }
}

impl PartialOrd for Size {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.width < other.width && self.height < other.height {
            Some(Ordering::Less)
        } else if self.width >= other.width && self.height >= other.height {
            Some(Ordering::Greater)
        } else {
            let self_area = self.area();
            let other_area = other.area();
            match self_area.partial_cmp(&other_area) {
                Some(Ordering::Less) => Some(Ordering::Less),
                Some(Ordering::Greater) => Some(Ordering::Greater),
                Some(Ordering::Equal) => self.width.partial_cmp(&other.width),
                None => None,
            }
        }
    }
}

impl Size {
    pub fn aligned_down_to(&self, h_alignment: u32, v_alignment: u32) -> Size {
        let width = self.width / h_alignment * h_alignment;
        let height = self.height / v_alignment * v_alignment;
        Size { width, height }
    }

    pub fn aligned_up_to(&self, h_alignment: u32, v_alignment: u32) -> Size {
        let width = (self.width + h_alignment - 1) / h_alignment * h_alignment;
        let height = (self.height + v_alignment - 1) / v_alignment * v_alignment;
        Size { width, height }
    }

    pub fn bounded_to(&self, bound: &Size) -> Size {
        let width = std::cmp::min(self.width, bound.width);
        let height = std::cmp::min(self.height, bound.height);
        Size { width, height }
    }

    pub fn expanded_to(&self, expand: &Size) -> Size {
        let width = std::cmp::max(self.width, expand.width);
        let height = std::cmp::max(self.height, expand.height);
        Size { width, height }
    }

    pub fn grown_by(&self, margins: &Size) -> Size {
        let width = self.width.saturating_add(margins.width);
        let height = self.height.saturating_add(margins.height);
        Size { width, height }
    }

    pub fn shrunk_by(&self, margins: &Size) -> Size {
        let width = self.width.saturating_sub(margins.width);
        let height = self.height.saturating_sub(margins.height);
        Size { width, height }
    }

    //noinspection DuplicatedCode
    //noinspection DuplicatedCode
    pub fn bounded_to_aspect_ratio(&self, ratio: &Size) -> Size {
        let ratio1: u64 = (self.width as u64) * (ratio.height as u64);
        let ratio2: u64 = (self.height as u64) * (ratio.width as u64);

        if ratio1 > ratio2 {
            Size {
                width: (ratio2 / ratio.height as u64) as u32,
                height: self.height,
            }
        } else {
            Size {
                width: self.width,
                height: (ratio1 / ratio.width as u64) as u32,
            }
        }
    }

    //noinspection DuplicatedCode
    //noinspection DuplicatedCode
    pub fn expanded_to_aspect_ratio(&self, ratio: &Size) -> Size {
        let ratio1: u64 = (self.width as u64) * (ratio.height as u64);
        let ratio2: u64 = (self.height as u64) * (ratio.width as u64);

        if ratio1 < ratio2 {
            Size {
                width: (ratio2 / ratio.height as u64) as u32,
                height: self.height,
            }
        } else {
            Size {
                width: self.width,
                height: (ratio1 / ratio.width as u64) as u32,
            }
        }
    }

    pub fn center_to(&self, center: Point) -> Rectangle {
        let x = center.x - (self.width / 2) as i32;
        let y = center.y - (self.height / 2) as i32;
        Rectangle {
            x,
            y,
            width: self.width,
            height: self.height,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.width == 0 && self.height == 0
    }

    pub fn area(&self) -> u64 {
        (self.width as u64) * (self.height as u64)
    }
}

impl From<libcamera_size_t> for Size {
    fn from(s: libcamera_size_t) -> Self {
        Self {
            width: s.width,
            height: s.height,
        }
    }
}

impl From<Size> for libcamera_size_t {
    fn from(s: Size) -> Self {
        Self {
            width: s.width,
            height: s.height,
        }
    }
}

/// Represents `libcamera::SizeRange`
#[derive(Debug, Clone, Copy)]
pub struct SizeRange {
    pub min: Size,
    pub max: Size,
    pub h_step: u32,
    pub v_step: u32,
}

impl SizeRange {
    pub fn contains(&self, size: &Size) -> bool {
        let width_out_of_bounds = (size.width < self.min.width) || (size.width > self.max.width);
        let height_out_of_bounds = (size.height < self.min.height) || (size.height > self.max.height);
        let width_not_in_steps = self.h_step > 0 && (size.width - self.min.width) % self.h_step != 0;
        let height_not_in_steps = self.v_step > 0 && (size.height - self.min.height) % self.v_step != 0;

        !(width_out_of_bounds || height_out_of_bounds || width_not_in_steps || height_not_in_steps)
    }
}

impl PartialEq for SizeRange {
    fn eq(&self, other: &Self) -> bool {
        self.min == other.min && self.max == other.max
    }
}

impl From<libcamera_size_range_t> for SizeRange {
    fn from(r: libcamera_size_range_t) -> Self {
        Self {
            min: r.min.into(),
            max: r.max.into(),
            h_step: r.hStep,
            v_step: r.vStep,
        }
    }
}

impl From<SizeRange> for libcamera_size_range_t {
    fn from(r: SizeRange) -> Self {
        Self {
            min: r.min.into(),
            max: r.max.into(),
            hStep: r.h_step,
            vStep: r.v_step,
        }
    }
}

/// Represents `libcamera::Rectangle`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn size(&self) -> Size {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    pub fn top_left(&self) -> Point {
        Point { x: self.x, y: self.y }
    }

    pub fn center(&self) -> Point {
        Point {
            x: self.x + (self.width / 2) as i32,
            y: self.y + (self.height / 2) as i32,
        }
    }

    pub fn scaled_by(&self, numerator: &Size, denominator: &Size) -> Rectangle {
        let x = (self.x as i64) * (numerator.width as i64) / (denominator.width as i64);
        let y = (self.y as i64) * (numerator.height as i64) / (denominator.height as i64);
        let width = (self.width as u64) * (numerator.width as u64) / (denominator.width as u64);
        let height = (self.height as u64) * (numerator.height as u64) / (denominator.height as u64);

        Rectangle {
            x: x as i32,
            y: y as i32,
            width: width as u32,
            height: height as u32,
        }
    }

    pub fn translated_by(&self, point: &Point) -> Rectangle {
        Rectangle {
            x: self.x + point.x,
            y: self.y + point.y,
            width: self.width,
            height: self.height,
        }
    }

    pub fn bounded_to(&self, bound: &Rectangle) -> Rectangle {
        let top_left_x = std::cmp::max(self.x, bound.x);
        let top_left_y = std::cmp::max(self.y, bound.y);
        let bottom_right_x = std::cmp::min(self.x + self.width as i32, bound.x + bound.width as i32);
        let bottom_right_y = std::cmp::min(self.y + self.height as i32, bound.y + bound.height as i32);

        let width: u32 = std::cmp::max(bottom_right_x - top_left_x, 0) as u32;
        let height: u32 = std::cmp::max(bottom_right_y - top_left_y, 0) as u32;

        Rectangle {
            x: top_left_x,
            y: top_left_y,
            width,
            height,
        }
    }

    pub fn enclosed_in(&self, boundary: Rectangle) -> Rectangle {
        let mut result = self.bounded_to(&Rectangle {
            x: self.x,
            y: self.y,
            width: boundary.width,
            height: boundary.height,
        });

        let max_x = boundary.x + boundary.width as i32 - result.width as i32;
        result.x = result.x.clamp(boundary.x, max_x);

        let max_y = boundary.y + boundary.height as i32 - result.height as i32;
        result.y = result.y.clamp(boundary.y, max_y);
        result
    }

    pub fn transformed_between(&self, src: &Rectangle, dst: &Rectangle) -> Rectangle {
        let sx: f64 = dst.width as f64 / src.width as f64;
        let sy: f64 = dst.height as f64 / src.height as f64;
        Rectangle {
            x: ((self.x - src.x) as f64 * sx) as i32 + dst.x,
            y: ((self.y - src.y) as f64 * sy) as i32 + dst.y,
            width: ((self.width as f64) * sx) as u32,
            height: ((self.height as f64) * sy) as u32,
        }
    }
}

impl Add<&Point> for Rectangle {
    type Output = Rectangle;

    fn add(self, point: &Point) -> Self::Output {
        self.translated_by(point)
    }
}

impl From<libcamera_rectangle_t> for Rectangle {
    fn from(r: libcamera_rectangle_t) -> Self {
        Self {
            x: r.x,
            y: r.y,
            width: r.width,
            height: r.height,
        }
    }
}

impl From<Rectangle> for libcamera_rectangle_t {
    fn from(r: Rectangle) -> Self {
        Self {
            x: r.x,
            y: r.y,
            width: r.width,
            height: r.height,
        }
    }
}
