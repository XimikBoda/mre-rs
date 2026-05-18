use crate::ffi::shape::*;
use crate::graphics::color::Color;
use crate::graphics::layer::{Layer};
use crate::graphics::canvas::Canvas;
use crate::graphics::color::set_global_color;
use alloc::vec::Vec;

pub trait DrawTarget {
    fn raw_buffer_ptr(&self) -> *mut u8;
}

impl DrawTarget for Layer {
    fn raw_buffer_ptr(&self) -> *mut u8 { self.buffer_ptr() }
}

impl DrawTarget for Canvas {
    fn raw_buffer_ptr(&self) -> *mut u8 { self.handle() as *mut u8 }
}

pub use crate::ffi::shape::vm_graphic_gp_style as GradientStyle;

pub fn set_pixel<T: DrawTarget>(target: &T, x: i32, y: i32, color: Color) {
    unsafe { vm_graphic_set_pixel(target.raw_buffer_ptr(), x, y, color.0) }
}

pub fn line<T: DrawTarget>(target: &T, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
    unsafe { vm_graphic_line(target.raw_buffer_ptr(), x0, y0, x1, y1, color.0) }
}

pub fn rect<T: DrawTarget>(target: &T, x: i32, y: i32, width: i32, height: i32, color: Color) {
    unsafe { vm_graphic_rect(target.raw_buffer_ptr(), x, y, width, height, color.0) }
}

pub fn fill_rect<T: DrawTarget>(target: &T, x: i32, y: i32, width: i32, height: i32, border_color: Color, fill_color: Color) {
    unsafe { vm_graphic_fill_rect(target.raw_buffer_ptr(), x, y, width, height, border_color.0, fill_color.0) }
}

pub fn round_rect<T: DrawTarget>(target: &T, x: i32, y: i32, width: i32, height: i32, corner_radius: i32, color: Color) {
    unsafe { vm_graphic_roundrect(target.raw_buffer_ptr(), x, y, width, height, corner_radius, color.0) }
}

pub fn fill_round_rect<T: DrawTarget>(target: &T, x: i32, y: i32, width: i32, height: i32, corner_radius: i32, color: Color) {
    unsafe { vm_graphic_fill_roundrect(target.raw_buffer_ptr(), x, y, width, height, corner_radius, color.0) }
}

pub fn ellipse<T: DrawTarget>(target: &T, x: i32, y: i32, width: i32, height: i32, color: Color) {
    unsafe { vm_graphic_ellipse(target.raw_buffer_ptr(), x, y, width, height, color.0) }
}

pub fn fill_ellipse<T: DrawTarget>(target: &T, x: i32, y: i32, width: i32, height: i32, color: Color) {
    unsafe { vm_graphic_fill_ellipse(target.raw_buffer_ptr(), x, y, width, height, color.0) }
}

pub fn polygon(layer: &Layer, points: &[(i16, i16)], color: Color) {
    let mre_points: Vec<vm_graphic_point> = points
        .iter()
        .map(|&(x, y)| vm_graphic_point { x, y })
        .collect();

    unsafe {
        let _ = set_global_color(color);
        vm_graphic_polygon(layer.handle(), mre_points.as_ptr(), mre_points.len() as i32);
    }
}

pub fn fill_polygon(layer: &Layer, points: &[(i16, i16)], fill_color: Color) {
    let mre_points: Vec<vm_graphic_point> = points
        .iter()
        .map(|&(x, y)| vm_graphic_point { x, y })
        .collect();

    unsafe {
        let _ = set_global_color(fill_color);
        vm_graphic_fill_polygon(layer.handle(), mre_points.as_ptr(), mre_points.len() as i32);
    }
}

pub fn gradient_rect(
    layer: &Layer, 
    x1: i32, y1: i32, 
    x2: i32, y2: i32, 
    color_start: Color, 
    color_end: Color, 
    style: GradientStyle
) {
    unsafe {
        vm_graphic_gradient_paint_rect(
            layer.handle(),
            x1, y1, x2, y2,
            color_start.into_sys_color(),
            color_end.into_sys_color(),
            style
        );
    }
}