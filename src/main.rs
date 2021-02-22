use tiny_render_rust::image::{Image, Color};
use tiny_render_rust::obj::ModelObject;
use tiny_render_rust::vec::{Point};
use rand::Rng;

fn main() {
    const WIDTH: u32 = 640;
    const HEIGHT: u32 = 480;
    let mut rng = rand::thread_rng();
    let model = ModelObject::from_file("objs/african_head.obj").unwrap();
    let mut img = tiny_render_rust::image::TGAImage::new(WIDTH, HEIGHT);
    println!("Load model successfully: #v {} #f {}", model.count_vertices(), model.count_faces());
    for face_index in 0..model.count_faces() {
        let face = model.get_face(face_index);
        let mut pts = vec![Point::new_empty(); 3];
        for j in 0..3 {
            let world_coords = model.get_vertex(face[j]);
            pts[j] = Point::new(
                ((world_coords.0 + 1.0) * WIDTH as f64 / 2.0) as u32,
                ((world_coords.1 + 1.0) * HEIGHT as f64 / 2.0) as u32,
            );
        }
        img.draw_triangle(&pts, Color::new(rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>() ));
    }
    img.write_to_file("test.tga").unwrap();
}
