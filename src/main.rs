use tiny_render_rust::image::{Image, Color};
use tiny_render_rust::obj::ModelObject;

fn main() {
    const WIDTH: u32 = 640;
    const HEIGHT: u32 = 480;
    let model = ModelObject::from_file("objs/african_head.obj").unwrap();
    let mut img = tiny_render_rust::image::TGAImage::new(WIDTH, HEIGHT);
    println!("Load model successfully: #v {} #f {}", model.count_vertices(), model.count_faces());
    img.draw_triangle(&mut [10, 70], &mut [50, 160], &mut [70, 80], Color::new(128, 0, 0));
    img.draw_triangle(&mut [180, 50], &mut [150, 1], &mut [70, 180], Color::new(255, 255, 255));
    img.draw_triangle(&mut [180, 150], &mut [120, 160], &mut [130, 180], Color::new(0, 128, 0));
    img.write_to_file("test.tga").unwrap();
}
