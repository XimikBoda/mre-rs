use crate::ffi::text::*;
use crate::ffi::ucs2::to_ucs2;
use crate::graphics::color::Color;
use crate::graphics::shape::DrawTarget;
use crate::graphics::layer::Layer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontSize {
    Small,
    Medium,
    Large,
}

impl FontSize {
    pub(crate) fn to_mre(self) -> vm_graphic_font_size {
        match self {
            FontSize::Small => vm_graphic_font_size::VM_SMALL_FONT,
            FontSize::Medium => vm_graphic_font_size::VM_MEDIUM_FONT,
            FontSize::Large => vm_graphic_font_size::VM_LARGE_FONT,
        }
    }
}

pub fn set_font_size(size: FontSize) {
    unsafe { vm_graphic_set_font(size.to_mre()) }
}

pub fn set_font_style(bold: bool, italic: bool, underline: bool) {
    unsafe {
        vm_font_set_font_style(
            if bold { 1 } else { 0 },
            if italic { 1 } else { 0 },
            if underline { 1 } else { 0 },
        );
    }
}

pub fn is_vector_font_supported() -> bool {
    unsafe { vm_graphic_is_use_vector_font() != 0 }
}

pub fn measure_string(text: &str) -> (i32, i32) {
    let utf16_str = to_ucs2(text);
    let width = unsafe { vm_graphic_get_string_width(utf16_str.as_ptr()) };
    let height = unsafe { vm_graphic_get_string_height(utf16_str.as_ptr()) };
    (width, height)
}

pub fn get_string_baseline(text: &str) -> i32 {
    let utf16_str = to_ucs2(text);
    unsafe { vm_graphic_get_string_baseline(utf16_str.as_ptr()) }
}

pub fn get_character_height() -> i32 {
    unsafe { vm_graphic_get_character_height() }
}

pub fn draw_text<T: DrawTarget>(target: &T, x: i32, y: i32, text: &str, color: Color) {
    let utf16_str = to_ucs2(text);
    
    let len = (utf16_str.len() - 1) as i32; 

    unsafe {
        vm_graphic_textout(
            target.raw_buffer_ptr(),
            x, 
            y,
            utf16_str.as_ptr(),
            len,
            color.0 
        );
    }
}

pub fn draw_text_by_baseline<T: DrawTarget>(
    target: &T, 
    x: i32, y: i32, 
    text: &str, 
    color: Color, 
    baseline: i32
) {
    let utf16_str = to_ucs2(text);
    let len = (utf16_str.len() - 1) as i32; 

    unsafe {
        vm_graphic_textout_by_baseline(
            target.raw_buffer_ptr(),
            x, 
            y,
            utf16_str.as_ptr(),
            len,
            color.0,
            baseline
        );
    }
}

pub fn draw_truncated_text(
    layer: &Layer, 
    x: i32, y: i32, 
    max_width: i32, 
    text: &str, 
    color: Color
) {
    let utf16_str = to_ucs2(text);
    let dot_symbol = to_ucs2("...");

    unsafe {
        vm_graphic_show_truncated_text(
            layer.handle(),
            x, 
            y,
            max_width,
            utf16_str.as_ptr(),
            dot_symbol.as_ptr(),
            0,
            color.0
        );
    }
}