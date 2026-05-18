use crate::mre_api;

pub const VM_BASE_LAYER: i32 = 1;
pub const VM_FAST_LAYER: i32 = 2;
pub const VM_BUF: i32 = 3;
pub const VM_CREATE_CANVAS: i32 = 4;

mre_api!(vm_graphic_create_layer(x: i32, y: i32, width: i32, height: i32, trans_color: i32) -> i32 = -1);
mre_api!(vm_graphic_create_layer_ex(x: i32, y: i32, width: i32, height: i32, trans_color: i32, mode: i32, buf: *mut u8) -> i32 = -1);

mre_api!(vm_graphic_delete_layer(handle: i32) -> i32 = -1);
mre_api!(vm_graphic_active_layer(handle: i32) -> i32 = -1);

mre_api!(vm_graphic_flush_layer(layer_handles: *const i32, count: i32) -> i32 = -1);

mre_api!(vm_graphic_get_layer_buffer(handle: i32) -> *mut u8);

mre_api!(vm_graphic_get_layer_position(handle: i32, x: *mut i32, y: *mut i32, width: *mut i32, height: *mut i32) -> i32 = -1);
mre_api!(vm_graphic_resize_layer(handle: i32, width: i32, height: i32) -> i32 = -1);
mre_api!(vm_graphic_translate_layer(handle: i32, tx: i32, ty: i32) -> i32 = -1);

mre_api!(vm_graphic_set_layer_opacity(handle: i32, percent: i32) -> i32 = -1);
mre_api!(vm_graphic_set_alpha_blending_layer(layer_handle: i32) -> i32 = -1);

mre_api!(vm_graphic_clear_layer_bg(handle: i32) -> i32 = -1);