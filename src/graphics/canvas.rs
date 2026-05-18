use crate::ffi::canvas::*;
use crate::ffi::screen::VM_NO_TRANS_COLOR;
use crate::graphics::color::Color;
use crate::graphics::layer::{Layer, LayerMode};

pub struct Canvas {
    handle: i32,
    width: i32,
    height: i32,
}

impl Canvas {
    pub fn create(width: i32, height: i32) -> Result<Self, i32> {
        let handle = unsafe { vm_graphic_create_canvas(width, height) };
        if handle <= 0 {
            return Err(handle);
        }
        Ok(Self { handle, width, height })
    }

    pub(crate) fn from_handle(handle: i32, width: i32, height: i32) -> Self {
        Self { handle, width, height }
    }

    #[inline]
    pub fn handle(&self) -> i32 {
        self.handle
    }

    pub fn pixel_buffer_ptr(&self) -> *mut u8 {
        unsafe {
            let ptr = vm_graphic_get_img_buffer(self.handle, 1);
            if !ptr.is_null() {
                ptr
            } else {
                //let base_ptr = vm_graphic_get_canvas_buffer(self.handle);
                let base_ptr = self.handle as *mut u8;
                base_ptr.add(VM_CANVAS_DATA_OFFSET)
            }
        }
    }

    pub fn pixels_mut(&mut self) -> Option<&mut [Color]> {
        let ptr = self.pixel_buffer_ptr();
        if ptr.is_null() {
            return None;
        }
        
        let color_ptr = ptr as *mut Color;
        let len = (self.width * self.height) as usize;
        
        unsafe {
            Some(core::slice::from_raw_parts_mut(color_ptr, len))
        }
    }

    pub fn frame_count(&self) -> i32 {
        unsafe { vm_graphic_get_frame_number(self.handle) }
    }

    pub fn frame_property(&self, frame_index: u8) -> Option<frame_prop> {
        let ptr = unsafe { vm_graphic_get_img_property(self.handle, frame_index) };
        if ptr.is_null() {
            None
        } else {
            unsafe { Some(*ptr) }
        }
    }

    pub fn set_trans_color(&self, color: Option<Color>) -> Result<(), i32> {
        let tc = color.map(|c| c.0 as i32).unwrap_or(VM_NO_TRANS_COLOR);
        let res = unsafe { vm_graphic_canvas_set_trans_color(self.handle, tc) };
        if res == 0 { Ok(()) } else { Err(res) }
    }

    pub fn bind_to_layer(&self, x: i32, y: i32, trans_color: Option<Color>) -> Result<Layer, i32> {
        Layer::create_ex(
            x, 
            y, 
            self.width, 
            self.height, 
            trans_color, 
            LayerMode::Buffer, 
            self.pixel_buffer_ptr()
        )
    }
}

impl Drop for Canvas {
    fn drop(&mut self) {
        unsafe {
            vm_graphic_release_canvas_ex(self.handle);
        }
    }
}