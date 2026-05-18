#![allow(non_camel_case_types)]
use crate::mre_api;
use crate::ffi::screen::vm_graphic_color;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct vm_graphic_point {
    pub x: i16,
    pub y: i16,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum vm_graphic_gp_style {
    VM_GDI_GP_HOR = 0,
    VM_GDI_GP_VER = 1,
    VM_GDI_GP_DIA = 2,
    VM_GDI_GP_DIA_INV = 3,
    VM_GDI_GP_FLIP = 4,
}

mre_api!(vm_graphic_set_pixel(buf: *mut u8, x: i32, y: i32, color: u16) -> ());
mre_api!(vm_graphic_line(buf: *mut u8, x0: i32, y0: i32, x1: i32, y1: i32, color: u16) -> ());
mre_api!(vm_graphic_rect(buf: *mut u8, x: i32, y: i32, width: i32, height: i32, color: u16) -> ());
mre_api!(vm_graphic_fill_rect(buf: *mut u8, x: i32, y: i32, width: i32, height: i32, line_color: u16, back_color: u16) -> ());
mre_api!(vm_graphic_roundrect(buf: *mut u8, x: i32, y: i32, width: i32, height: i32, corner_width: i32, color: u16) -> ());
mre_api!(vm_graphic_fill_roundrect(buf: *mut u8, x: i32, y: i32, width: i32, height: i32, corner_width: i32, color: u16) -> ());
mre_api!(vm_graphic_ellipse(buf: *mut u8, x: i32, y: i32, width: i32, height: i32, color: u16) -> ());
mre_api!(vm_graphic_fill_ellipse(buf: *mut u8, x: i32, y: i32, width: i32, height: i32, color: u16) -> ());

mre_api!(vm_graphic_polygon(handle: i32, point: *const vm_graphic_point, npoint: i32) -> ());
mre_api!(vm_graphic_fill_polygon(handle: i32, point: *const vm_graphic_point, npoints: i32) -> ());
mre_api!(vm_graphic_gradient_paint_rect(handle: i32, x1: i32, y1: i32, x2: i32, y2: i32, color_start: vm_graphic_color, color_end: vm_graphic_color, style: vm_graphic_gp_style) -> ());