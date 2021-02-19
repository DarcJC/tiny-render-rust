use tiny_render_rust::image::{Image, Color};
use tiny_render_rust::obj::ModelObject;

fn main() {
    let mut model = ModelObject::from_file("objs/african_head.obj").unwrap();
    println!("Load model successfully: #v {} #f {}", model.count_vertices(), model.count_faces());
    let mut img = tiny_render_rust::image::TGAImage::new(640, 480);
    img.draw_line(13, 20, 80, 40, Color::new(255,255,255));
    img.draw_line(20, 13, 40, 80, Color::new(255,0,0));
    img.draw_line(80, 40, 13, 20, Color::new(255,0,0));
    img.apply_gamma(2.2);
    img.write_to_file("test.tga").unwrap();
}
