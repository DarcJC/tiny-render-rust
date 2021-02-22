use tiny_render_rust::image::{Image, Color};
use tiny_render_rust::obj::ModelObject;
use tiny_render_rust::vec::{Point, Vec3};

fn main() {
    const WIDTH: u32 = 640;
    const HEIGHT: u32 = 480;
    let light_dir: Vec3<f64> = Vec3::new(1f64, 1f64, -1f64).normalize();

    let model = ModelObject::from_file("objs/african_head.obj").unwrap();
    let mut img = tiny_render_rust::image::TGAImage::new(WIDTH, HEIGHT);
    println!("Load model successfully: #v {} #f {}", model.count_vertices(), model.count_faces());
    for face_index in 0..model.count_faces() {
        let face = model.get_face(face_index);
        let mut pts = vec![Point::new_empty(); 3];
        let mut world_coords = vec![Vec3::new(0f64, 0f64, 0f64); 3];
        for j in 0..3 {
            let world_coord = model.get_vertex(face[j]);
            world_coords[j] = Vec3::new(world_coord.0, world_coord.1, world_coord.2);
            pts[j] = Point::new(
                ((world_coord.0 + 1.0) * WIDTH as f64 / 2.0) as u32,
                ((world_coord.1 + 1.0) * HEIGHT as f64 / 2.0) as u32,
            );
        }
        let normal = Vec3::cross(&(world_coords[2] - world_coords[0]), &(world_coords[1] - world_coords[0])).normalize();
        let intensity = normal * light_dir;
        let triangle_color = (intensity * 255.0) as u8;

        if intensity > 0.0 {
            img.draw_triangle(&pts, Color::new(triangle_color, triangle_color, triangle_color));
        }
    }
    img.write_to_file("test.tga").unwrap();
}
