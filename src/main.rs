use tiny_render_rust::image::{Image, Color};
use tiny_render_rust::obj::ModelObject;
use tiny_render_rust::vec::{Point};

fn main() {
    const WIDTH: u32 = 640;
    const HEIGHT: u32 = 480;
    let model = ModelObject::from_file("objs/african_head.obj").unwrap();
    let mut img = tiny_render_rust::image::TGAImage::new(WIDTH, HEIGHT);
    println!("Load model successfully: #v {} #f {}", model.count_vertices(), model.count_faces());
    let pts = vec![
        Point::new(10, 10),
        Point::new(100, 30),
        Point::new(190, 160),
    ];
    img.draw_triangle(&pts, Color::new(128, 0, 0));
    img.write_to_file("test.tga").unwrap();
}
