use crate::ffi::screen::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Color(pub u16);

impl Color {
    pub const BLACK: Self      = Self(0x0000);
    pub const WHITE: Self      = Self(0xFFFF);
    pub const RED: Self        = Self(0xF800);
    pub const GREEN: Self      = Self(0x07E0);
    pub const BLUE: Self       = Self(0x001F);
    
    pub const YELLOW: Self     = Self(0xFFE0);
    pub const CYAN: Self       = Self(0x07FF); 
    pub const MAGENTA: Self    = Self(0xF81F); 

    pub const GRAY: Self       = Self(0x8410);
    pub const LIGHT_GRAY: Self = Self(0xC618);
    pub const DARK_GRAY: Self  = Self(0x4208);

    pub const ORANGE: Self     = Self(0xFD20);
    pub const PURPLE: Self     = Self(0x8010);
    pub const PINK: Self       = Self(0xFE19);
    pub const BROWN: Self      = Self(0xA145);
    pub const GOLD: Self       = Self(0xFEA0);

    pub const MAROON: Self     = Self(0x8000);
    pub const DARK_GREEN: Self = Self(0x0400);
    pub const NAVY: Self       = Self(0x0010);

    #[inline]
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        let r5 = (r >> 3) as u16;
        let g6 = (g >> 2) as u16;
        let b5 = (b >> 3) as u16;
        
        Self((r5 << 11) | (g6 << 5) | b5)
    }

    pub const fn blend(bg: Color, fg: Color, alpha: u8) -> Self {
        if alpha == 0 { return bg; }
        if alpha == 255 { return fg; }

        let a = alpha as u16;
        let inv_a = 255 - a;

        let r = ((fg.r() as u16 * a + bg.r() as u16 * inv_a) / 255) as u8;
        let g = ((fg.g() as u16 * a + bg.g() as u16 * inv_a) / 255) as u8;
        let b = ((fg.b() as u16 * a + bg.b() as u16 * inv_a) / 255) as u8;

        Self::from_rgb(r, g, b)
    }

    pub const fn blend_fast(bg_pixel: Color, fg_pixel: Color, alpha: u8) -> Self {
        if alpha == 0 { return bg_pixel; }
        if alpha >= 248 { return fg_pixel; }

        let a = (alpha >> 3) as u32;

        let mut fg = fg_pixel.0 as u32;
        let mut bg = bg_pixel.0 as u32;

        fg = (fg | (fg << 16)) & 0x07E0_F81F;
        bg = (bg | (bg << 16)) & 0x07E0_F81F;

        let diff = fg.wrapping_sub(bg);
        let offset = diff.wrapping_mul(a) >> 5;
        bg = bg.wrapping_add(offset);

        bg &= 0x07E0_F81F;

        Self((bg | (bg >> 16)) as u16)
    }


    #[inline]
    pub const fn from_rgb_rounded(r: u8, g: u8, b: u8) -> Self {
        let r5 = (r.saturating_add(4) >> 3) as u16;
        
        let g6 = (g.saturating_add(2) >> 2) as u16;
        
        let b5 = (b.saturating_add(4) >> 3) as u16;
        
        Self((r5 << 11) | (g6 << 5) | b5)
    }

    #[inline]
    pub const fn r(&self) -> u8 {
        ((self.0 & 0xF800) >> 8) as u8
    }

    #[inline]
    pub const fn g(&self) -> u8 {
        ((self.0 & 0x07E0) >> 3) as u8
    }

    #[inline]
    pub const fn b(&self) -> u8 {
        ((self.0 & 0x001F) << 3) as u8
    }

    pub(crate) fn into_sys_color(self) -> vm_graphic_color {
        let c = self.0 as u32;
        let color888 = ((c & 0x001F) << 19) | ((c & 0x07E0) << 5) | ((c & 0xF800) >> 8);
        
        vm_graphic_color {
            vm_color_565: c,
            vm_color_888: color888,
        }
    }

    pub(crate) fn from_sys_color(sys: vm_graphic_color) -> Self {
        Self(sys.vm_color_565 as u16)
    }
}

pub fn set_global_color(color: Color) -> Result<(), i32> {
    let mut sys_color = color.into_sys_color();
    unsafe {
        let res = vm_graphic_setcolor(&mut sys_color);
        if res == 0 {
            Ok(())
        } else {
            Err(res)
        }
    }
}

pub fn get_global_color() -> Color {
    unsafe {
        let sys_color = vm_graphic_getcolor();
        Color::from_sys_color(sys_color)
    }
}