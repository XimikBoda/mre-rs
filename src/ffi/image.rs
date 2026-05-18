#![allow(non_snake_case)]
use crate::mre_api;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct vm_graphic_imgprop {
    pub width: i32,
    pub height: i32,
}

mre_api!(vm_graphic_load_image(img: *const u8, img_len: i32) -> i32 = -1);
mre_api!(vm_graphic_load_image_resized(img: *const u8, img_len: i32, width: i32, height: i32) -> i32 = -1);

mre_api!(vm_graphic_get_img_property_ex(img_data: *const u8, img_len: i32, img_prop: *mut vm_graphic_imgprop) -> i32 = -500);
mre_api!(vm_graphic_get_img_property_from_file(filename: *const u16, img_prop: *mut vm_graphic_imgprop) -> i32);

mre_api!(vm_graphic_draw_image_from_memory(dest_layer_handle: i32, x: i32, y: i32, img_data: *const u8, img_len: i32) -> i32 = -1);
mre_api!(vm_graphic_draw_resized_image_from_memory(dest_layer_handle: i32, x: i32, y: i32, resized_width: i32, resized_height: i32, img_data: *const u8, img_len: i32) -> i32 = -1);

mre_api!(vm_graphic_draw_gif_image_from_memory(dest_layer_handle: i32, x: i32, y: i32, img_data: *const u8, img_len: i32, frameIdx: i32) -> i32 = -1);
mre_api!(vm_graphic_draw_resized_gif_image_from_memory(dest_layer_handle: i32, x: i32, y: i32, resized_width: i32, resized_height: i32, img_data: *const u8, img_len: i32, frameIdx: i32) -> i32 = -1);

mre_api!(vm_graphic_draw_image_from_file(dest_layer_handle: i32, x: i32, y: i32, filename: *const u16) -> i32 = -1);
mre_api!(vm_graphic_draw_resized_image_from_file(dest_layer_handle: i32, x: i32, y: i32, resized_width: i32, resized_height: i32, filename: *const u16) -> i32 = -1);