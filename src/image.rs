use std::io;
use std::fs::File;
use std::io::Write;
use std::slice;
use std::mem;
use std::mem::swap;
use crate::vec::{Vec3, Point};

#[derive(Clone)]
pub struct Color(u8, u8, u8);

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self(b, g, r) // note.
    }
}

pub trait Image {
    fn new(width: u32, height: u32) -> Self;
    fn apply_gamma(self: &mut Self, gamma: f32);
    fn set_pixel(self: &mut Self, x: u32, y: u32, c: Color);
    fn write_to_file(self: &Self, filename: &str) -> io::Result<()>;
    fn draw_line(self: &mut Self, p0: Point, p1: Point, color: Color);
    fn draw_triangle(self: &mut Self, pts: &Vec<Point>, color: Color);
}

pub struct TGAImage {
    width: u32,
    height: u32,
    data: Vec<Color>,
}

pub fn barycentric(points: &Vec<Point>, p: Point) -> Vec3<f64> {
    let v1 = Vec3::new(
        points[2].x as f64 - points[0].x as f64,
        points[1].x as f64 - points[0].x as f64,
        points[0].x as f64 - p.x as f64,
    );
    let v2 = Vec3::new(
        points[2].y as f64 - points[0].y as f64,
        points[1].y as f64 - points[0].y as f64,
        points[0].y as f64 - p.y as f64,
    );
    let c = Vec3::cross(&v1, &v2);

    if c.z.abs() < 1.0 {
        Vec3::new(-1.0, 1.0, 1.0)
    } else {
        Vec3::new(1.0 - (c.x + c.y) / c.z, c.y / c.z, c.x / c.z)
    }
}

impl Image for TGAImage {
    fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            data: vec![Color(0, 0, 0); (width * height) as usize],
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
        if x + y * self.width > self.data.len() as u32 {
            println!("Draw over bounds: {}/{}", x + y * self.width, self.data.len());
            return;
        }
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
        }
        ;
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

    fn draw_line(self: &mut Self, mut p0: Point, mut p1: Point, color: Color) {
        let mut steep = false;
        if (p0.x as i32 - p1.x as i32).abs() < (p0.y as i32 - p1.y as i32).abs() {
            swap(&mut p0.x, &mut p0.y);
            swap(&mut p1.x, &mut p1.y);
            steep = true;
        }
        if p0.x > p1.x {
            swap(&mut p0, &mut p1);
        }
        for x in p0.x..=p1.x {
            let t = (x - p0.x) as f32 / (p1.x - p0.x) as f32;
            let y = p0.y as f32 * (1f32 - t) + p1.y as f32 * t;
            if steep {
                self.set_pixel(y as u32, x, color.clone());
            } else {
                self.set_pixel(x, y as u32, color.clone());
            }
        }
    }

    fn draw_triangle(self: &mut Self, pts: &Vec<Point>, color: Color) {
        let mut bboxmax = Point::new(0, 0);
        let mut bboxmin = Point::new(self.width - 1, self.height - 1);
        for p in pts {
            if p.x < bboxmin.x {
                bboxmin.x = p.x;
            }
            if p.y < bboxmin.y {
                bboxmin.y = p.y;
            }
            if p.x > bboxmax.x {
                bboxmax.x = p.x;
            }
            if p.y > bboxmax.y {
                bboxmax.y = p.y;
            }
        }
        for i in bboxmin.x ..= bboxmax.x {
            for j in bboxmin.y ..= bboxmax.y {
                let p = Point::new(i, j);
                let res = barycentric(&pts, p);
                if res.x >= 0.0 && res.y >= 0.0 && res.z >= 0.0 {
                    self.set_pixel(i, j, color.clone());
                }
            }
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
