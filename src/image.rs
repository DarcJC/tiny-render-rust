use std::io;
use std::fs::File;
use std::io::Write;
use core::{slice, mem};

#[derive(Clone)]
pub struct Color(u8, u8, u8);

impl Color {
    pub fn new(r: u8, g: u8, b:u8) -> Self {
        Self(r, g, b)
    }
}

pub trait Image {
    fn new(width: u32, height: u32) -> Self;
    fn apply_gamma(self: &mut Self, gamma: f32);
    fn set_pixel(self: &mut Self, x: u32, y: u32, c: Color);
    fn write_to_file(self: &Self, filename: &str) -> io::Result<()>;
    fn draw_line(self: &mut Self, x0: u32, y0: u32, x1: u32, y1: u32, color: Color);
}

pub struct TGAImage {
    width: u32,
    height: u32,
    data: Vec<Color>,
}

impl Image for TGAImage {
    fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            data: vec![Color(0,0,0); (width * height) as usize],
        }
    }

    fn apply_gamma(self: &mut Self, gamma: f32) {
        for c in self.data.iter_mut() {
            let Color(r, g, b) = *c;
            let fr = ((r as f32) / 255.0).powf(gamma);
            let fg = ((g as f32) / 255.0).powf(gamma);
            let fb = ((b as f32) / 255.0).powf(gamma);
            c.0 = (fr * 255.0) as u8;
            c.1 = (fg * 255.0) as u8;
            c.2 = (fb * 255.0) as u8;
        }
    }

    fn set_pixel(self: &mut Self, x: u32, y: u32, c: Color) {
        self.data[(x + y * self.width) as usize] = c;
    }

    fn write_to_file(self: &Self, filename: &str) -> io::Result<()> {
        #[repr(C, packed)]
        #[derive(Default)]
        struct TGAHeader {
            id_length: u8,
            color_map_type: u8,
            image_type: u8,
            c_map_start: u16,
            c_map_length: u16,
            c_map_depth: u8,
            x_offset: u16,
            y_offset: u16,
            width: u16,
            height: u16,
            pixel_depth: u8,
            image_descriptor: u8,
        };
        let h = TGAHeader {
            image_type: 2,
            width: self.width as u16,
            height: self.height as u16,
            pixel_depth: 24,
            ..TGAHeader::default()
        };
        let mut f = File::create(filename)?;
        unsafe {
            f.write_all(struct_to_u8_slice(&h))?;
            f.write_all(slice_to_u8_slice(&self.data[..]))?;
        };
        Ok(())
    }

    fn draw_line(self: &mut Self, x0: u32, y0: u32, x1: u32, y1: u32, color: Color) {
        let mut t= 0f32;
        while t < 1f32 {
            let x = (x0 + (x1 - x0)) as f32 * t;
            let y = (y0 + (y1 - y0)) as f32 * t;
            self.set_pixel(x as u32, y as u32, color.clone());
            t += 0.01f32
        }
    }
}

unsafe fn struct_to_u8_slice<T>(s: &T) -> &[u8] {
    let data_ptr: *const u8 = mem::transmute(s);
    slice::from_raw_parts(data_ptr, mem::size_of::<T>())
}

unsafe fn slice_to_u8_slice<T>(s: &[T]) -> &[u8] {
    let data_ptr: *const u8 = mem::transmute(&s[0]);
    slice::from_raw_parts(data_ptr, mem::size_of::<T>() * s.len())
}
