use crate::mre_api;

pub const VM_CANVAS_DATA_OFFSET: usize = 32;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct frame_prop {
    pub flag: u8,
    pub left: u16,
    pub top: u16,
    pub width: u16,
    pub height: u16,
    pub delay_time: u16,
    pub trans_color_index: u8,
    pub trans_color: u16,
    pub offset: u32,
}

mre_api!(vm_graphic_create_canvas(width: i32, height: i32) -> i32 = -1);
mre_api!(vm_graphic_get_canvas_buffer(hcanvas: i32) -> *mut u8);
mre_api!(vm_graphic_get_canvas_buffer_size(hcanvas: i32) -> i32 = -1);

mre_api!(vm_graphic_get_img_buffer(hcanvas: i32, frame_index: u8) -> *mut u8);
mre_api!(vm_graphic_get_img_property(hcanvas: i32, frame_index: u8) -> *const frame_prop);

mre_api!(vm_graphic_get_frame_number(hcanvas: i32) -> i32 = -1);

mre_api!(vm_graphic_release_canvas(hcanvas: i32) -> i32);
mre_api!(vm_graphic_release_canvas_ex(hcanvas: i32) -> i32 = -1);

mre_api!(vm_graphic_canvas_set_trans_color(hcanvas: i32, trans_color: i32) -> i32 = -1);