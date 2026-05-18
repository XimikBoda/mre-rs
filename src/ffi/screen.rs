use crate::mre_api;

mre_api!(vm_graphic_get_screen_width() -> i32 = -1);
mre_api!(vm_graphic_get_screen_height() -> i32 = -1);
mre_api!(vm_graphic_get_bits_per_pixel() -> i32 = -1);

mre_api!(vm_graphic_set_clip(x1: i32, y1: i32, x2: i32, y2: i32));
mre_api!(vm_graphic_reset_clip());

mre_api!(vm_graphic_flush_screen());

pub const VM_NO_TRANS_COLOR: i32 = -1;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct vm_graphic_color {
    pub vm_color_565: u32,
    pub vm_color_888: u32,
}

mre_api!(vm_graphic_setcolor(color: *mut vm_graphic_color) -> i32 = -1);
mre_api!(vm_graphic_getcolor() -> vm_graphic_color);