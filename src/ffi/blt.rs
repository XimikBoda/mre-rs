use crate::mre_api;

mre_api!(vm_graphic_blt(
    dst_disp_buf: *mut u8, x_dest: i32, y_dest: i32, 
    src_disp_buf: *mut u8, x_src: i32, y_src: i32, 
    width: i32, height: i32, frame_index: i32
));

mre_api!(vm_graphic_blt_ex(
    dst_disp_buf: *mut u8, x_dest: i32, y_dest: i32, 
    src_disp_buf: *mut u8, x_src: i32, y_src: i32, 
    width: i32, height: i32, frame_index: i32, alpha: i32
));

mre_api!(vm_graphic_rotate(
    buf: *mut u8, x_des: i32, y_des: i32, 
    src_buf: *mut u8, frame_index: i32, degrees: i32
));

mre_api!(vm_graphic_mirror(
    buf: *mut u8, x_des: i32, y_des: i32, 
    src_buf: *mut u8, frame_index: i32, direction: i32
));