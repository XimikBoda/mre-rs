use crate::ffi::image::*;
use crate::graphics::canvas::Canvas;
use crate::graphics::layer::Layer;
use crate::fs::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImageProperties {
    pub width: i32,
    pub height: i32,
}

pub fn get_properties(image_data: &[u8]) -> Result<ImageProperties, i32> {
    let mut prop = vm_graphic_imgprop { width: 0, height: 0 };
    let res = unsafe {
        vm_graphic_get_img_property_ex(
            image_data.as_ptr(), 
            image_data.len() as i32, 
            &mut prop
        )
    };

    if res == 0 { // VM_GDI_SUCCEED
        Ok(ImageProperties {
            width: prop.width,
            height: prop.height,
        })
    } else {
        Err(res)
    }
}

pub fn get_properties_from_file(path: &Path) -> Result<ImageProperties, i32> {
    let path_u16 = path.as_mre_str();
    let mut prop = vm_graphic_imgprop { width: 0, height: 0 };
    
    let res = unsafe {
        vm_graphic_get_img_property_from_file(path_u16.as_ptr(), &mut prop)
    };

    if res == 0 {
        Ok(ImageProperties {
            width: prop.width,
            height: prop.height,
        })
    } else {
        Err(res)
    }
}

pub fn load_canvas(image_data: &[u8]) -> Result<Canvas, i32> {
    let props = get_properties(image_data)?;

    let handle = unsafe { 
        vm_graphic_load_image(image_data.as_ptr(), image_data.len() as i32) 
    };

    if handle <= 0 {
        return Err(handle);
    }

    Ok(Canvas::from_handle(handle, props.width, props.height))
}

pub fn load_canvas_resized(image_data: &[u8], new_width: i32, new_height: i32) -> Result<Canvas, i32> {
    let handle = unsafe { 
        vm_graphic_load_image_resized(
            image_data.as_ptr(), 
            image_data.len() as i32, 
            new_width, 
            new_height
        ) 
    };

    if handle <= 0 {
        return Err(handle);
    }

    Ok(Canvas::from_handle(handle, new_width, new_height))
}

pub fn draw_to_layer(layer: &Layer, x: i32, y: i32, image_data: &[u8]) -> Result<(), i32> {
    let res = unsafe {
        vm_graphic_draw_image_from_memory(
            layer.handle(), 
            x, 
            y, 
            image_data.as_ptr(), 
            image_data.len() as i32
        )
    };
    if res == 0 { Ok(()) } else { Err(res) }
}

pub fn draw_resized_to_layer(
    layer: &Layer, 
    x: i32, y: i32, 
    new_width: i32, new_height: i32, 
    image_data: &[u8]
) -> Result<(), i32> {
    let res = unsafe {
        vm_graphic_draw_resized_image_from_memory(
            layer.handle(), 
            x, y, 
            new_width, new_height, 
            image_data.as_ptr(), 
            image_data.len() as i32
        )
    };
    if res == 0 { Ok(()) } else { Err(res) }
}

pub fn draw_gif_frame_to_layer(
    layer: &Layer, 
    x: i32, y: i32, 
    frame_index: i32, 
    image_data: &[u8]
) -> Result<(), i32> {
    let res = unsafe {
        vm_graphic_draw_gif_image_from_memory(
            layer.handle(), 
            x, y, 
            image_data.as_ptr(), 
            image_data.len() as i32, 
            frame_index
        )
    };
    if res == 0 { Ok(()) } else { Err(res) }
}

pub fn draw_resized_gif_frame_to_layer(
    layer: &Layer, 
    x: i32, y: i32, 
    new_width: i32, new_height: i32,
    frame_index: i32, 
    image_data: &[u8]
) -> Result<(), i32> {
    let res = unsafe {
        vm_graphic_draw_resized_gif_image_from_memory(
            layer.handle(), 
            x, y, 
            new_width, new_height, 
            image_data.as_ptr(), 
            image_data.len() as i32, 
            frame_index
        )
    };
    if res == 0 { Ok(()) } else { Err(res) }
}

pub fn draw_from_file(layer: &Layer, x: i32, y: i32, path: &Path) -> Result<(), i32> {
    let path_u16 = path.as_mre_str();
    let res = unsafe {
        vm_graphic_draw_image_from_file(
            layer.handle(), 
            x, 
            y, 
            path_u16.as_ptr()
        )
    };
    if res == 0 { Ok(()) } else { Err(res) }
}

pub fn draw_resized_from_file(
    layer: &Layer, 
    x: i32, y: i32, 
    new_width: i32, new_height: i32, 
    path: &Path
) -> Result<(), i32> {
    let path_u16 = path.as_mre_str();
    let res = unsafe {
        vm_graphic_draw_resized_image_from_file(
            layer.handle(), 
            x, y, 
            new_width, new_height, 
            path_u16.as_ptr()
        )
    };
    if res == 0 { Ok(()) } else { Err(res) }
}