use tiny_render_rust::image::{Image};

fn main() {
    let mut img = tiny_render_rust::image::TGAImage::new(640, 480);
    for y in 0u32..480 {
        for x in 0u32..640 {
            let r = ((x + y) % 256) as u8;
            let g = ((x ^ y) % 256) as u8;
            let b = ((y.wrapping_sub(x)) % 256) as u8;
            img.set_pixel(x, y, tiny_render_rust::image::Color::new(r, g, b));
        }
    }
    img.apply_gamma(2.2);
    img.write_to_file("test.tga").unwrap();
}
