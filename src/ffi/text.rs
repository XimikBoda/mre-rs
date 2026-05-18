#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use crate::mre_api;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum vm_graphic_font_size {
    VM_SMALL_FONT = 0,
    VM_MEDIUM_FONT = 1,
    VM_LARGE_FONT = 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct vm_graphic_char_info {
    pub dwidth: i32,
    pub width: i32,
    pub height: i32,
    pub ascent: i32,
    pub descent: i32,
}

mre_api!(vm_graphic_set_font(size: vm_graphic_font_size));
mre_api!(vm_font_set_font_size(size: i32) -> i32 = -1);
mre_api!(vm_font_set_font_style(bold: i32, italic: i32, underline: i32) -> i32 = -1);
mre_api!(vm_graphic_is_use_vector_font() -> i32 = -1);

mre_api!(vm_graphic_get_character_height() -> i32 = -1);
mre_api!(vm_graphic_get_character_width(c: u16) -> i32 = -1);
mre_api!(vm_graphic_get_string_width(str: *const u16) -> i32 = -1);
mre_api!(vm_graphic_get_string_height(str: *const u16) -> i32 = -1);
mre_api!(vm_graphic_get_string_baseline(str: *const u16) -> i32 = -1);
mre_api!(vm_graphic_measure_character(c: u16, width: *mut i32, height: *mut i32) -> i32 = -1);
mre_api!(vm_graphic_get_character_info(c: u16, char_info: *mut vm_graphic_char_info) -> i32 = -1);

mre_api!(vm_graphic_textout(disp_buf: *mut u8, x: i32, y: i32, s: *const u16, length: i32, color: u16));
mre_api!(vm_graphic_textout_by_baseline(disp_buf: *mut u8, x: i32, y: i32, s: *const u16, length: i32, color: u16, baseline: i32));

mre_api!(vm_graphic_show_truncated_text(dest_layer_handle: i32, x: i32, y: i32, xwidth: i32, st: *const u16, truncated_symbol: *const u16, bordered: i32, color: u16) -> i32 = -1);
mre_api!(vm_graphic_draw_abm_text(handle: i32, x: i32, y: i32, color: i32, font_data: *const u8, font_width: i32, font_height: i32) -> i32 = -1);

mre_api!(vm_get_string_width_height_ex(string: *const u16, gap: i32, n: i32, pWidth: *mut i32, pHeight: *mut i32, max_width: i32, checkLineBreak: u8, checkCompleteWord: u8) -> u32);