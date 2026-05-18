use crate::ffi::blt::*;
use crate::graphics::shape::DrawTarget;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotateDegree {
    Deg90 = 90,
    Deg180 = 180,
    Deg270 = 270,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MirrorDirection {
    Horizontal = 0,
    Vertical = 1,
}

pub fn blt<D: DrawTarget, S: DrawTarget>(
    dest: &D, dest_x: i32, dest_y: i32,
    src: &S, src_x: i32, src_y: i32,
    width: i32, height: i32
) {
    unsafe {
        vm_graphic_blt(
            dest.raw_buffer_ptr(), dest_x, dest_y,
            src.raw_buffer_ptr(), src_x, src_y,
            width, height, 
            1
        );
    }
}

pub fn blt_frame<D: DrawTarget, S: DrawTarget>(
    dest: &D, dest_x: i32, dest_y: i32,
    src: &S, src_x: i32, src_y: i32,
    width: i32, height: i32,
    frame_index: i32
) {
    unsafe {
        vm_graphic_blt(
            dest.raw_buffer_ptr(), dest_x, dest_y,
            src.raw_buffer_ptr(), src_x, src_y,
            width, height, 
            frame_index
        );
    }
}

pub fn blt_alpha<D: DrawTarget, S: DrawTarget>(
    dest: &D, dest_x: i32, dest_y: i32,
    src: &S, src_x: i32, src_y: i32,
    width: i32, height: i32,
    alpha: u8
) {
    unsafe {
        vm_graphic_blt_ex(
            dest.raw_buffer_ptr(), dest_x, dest_y,
            src.raw_buffer_ptr(), src_x, src_y,
            width, height, 
            1,
            alpha as i32
        );
    }
}

pub fn blt_frame_alpha<D: DrawTarget, S: DrawTarget>(
    dest: &D, dest_x: i32, dest_y: i32,
    src: &S, src_x: i32, src_y: i32,
    width: i32, height: i32,
    frame_index: i32,
    alpha: u8
) {
    unsafe {
        vm_graphic_blt_ex(
            dest.raw_buffer_ptr(), dest_x, dest_y,
            src.raw_buffer_ptr(), src_x, src_y,
            width, height, 
            frame_index, 
            alpha as i32
        );
    }
}

pub fn rotate<D: DrawTarget, S: DrawTarget>(
    dest: &D, dest_x: i32, dest_y: i32,
    src: &S, 
    degree: RotateDegree
) {
    unsafe {
        vm_graphic_rotate(
            dest.raw_buffer_ptr(), dest_x, dest_y,
            src.raw_buffer_ptr(), 
            1, 
            degree as i32
        );
    }
}

pub fn rotate_frame<D: DrawTarget, S: DrawTarget>(
    dest: &D, dest_x: i32, dest_y: i32,
    src: &S, 
    frame_index: i32, 
    degree: RotateDegree
) {
    unsafe {
        vm_graphic_rotate(
            dest.raw_buffer_ptr(), dest_x, dest_y,
            src.raw_buffer_ptr(), 
            frame_index, 
            degree as i32
        );
    }
}

pub fn mirror<D: DrawTarget, S: DrawTarget>(
    dest: &D, dest_x: i32, dest_y: i32,
    src: &S, 
    direction: MirrorDirection
) {
    unsafe {
        vm_graphic_mirror(
            dest.raw_buffer_ptr(), dest_x, dest_y,
            src.raw_buffer_ptr(), 
            1,
            direction as i32
        );
    }
}

pub fn mirror_frame<D: DrawTarget, S: DrawTarget>(
    dest: &D, dest_x: i32, dest_y: i32,
    src: &S, 
    frame_index: i32, 
    direction: MirrorDirection
) {
    unsafe {
        vm_graphic_mirror(
            dest.raw_buffer_ptr(), dest_x, dest_y,
            src.raw_buffer_ptr(), 
            frame_index, 
            direction as i32
        );
    }
}