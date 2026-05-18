use crate::ffi::screen::*;

#[inline]
pub fn width() -> i32 {
    unsafe { vm_graphic_get_screen_width() }
}

#[inline]
pub fn height() -> i32 {
    unsafe { vm_graphic_get_screen_height() }
}

#[inline]
pub fn bits_per_pixel() -> i32 {
    unsafe { vm_graphic_get_bits_per_pixel() }
}

#[inline]
pub fn set_clip(x1: i32, y1: i32, x2: i32, y2: i32) {
    unsafe {
        vm_graphic_set_clip(x1, y1, x2, y2);
    }
}

#[inline]
pub fn reset_clip() {
    unsafe {
        vm_graphic_reset_clip();
    }
}

#[inline]
pub fn request_paint() {
    unsafe {
        vm_graphic_flush_screen();
    }
}