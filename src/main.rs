use tiny_render_rust::image::{Image, Color};

fn main() {
    let mut img = tiny_render_rust::image::TGAImage::new(640, 480);
    img.draw_line(0, 0, 128, 128, Color::new(255, 255, 255));
    img.write_to_file("test.tga").unwrap();
}
