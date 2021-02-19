use tiny_render_rust::image::{Image, Color};
use tiny_render_rust::obj::ModelObject;

fn main() {
    const WIDTH: u32 = 640;
    const HEIGHT: u32 = 480;
    let model = ModelObject::from_file("objs/african_head.obj").unwrap();
    let mut img = tiny_render_rust::image::TGAImage::new(WIDTH, HEIGHT);
    println!("Load model successfully: #v {} #f {}", model.count_vertices(), model.count_faces());
    for i in 0..model.count_faces() {
        let f = model.get_face(i);
        for j in 0..3 {
            let v0 = model.get_vertex(f[j]);
            let v1 = model.get_vertex(f[ ( j + 1 ) % 3 ]);
            let x0 = (v0.0 + 1f64) * WIDTH as f64 / 2f64;
            let y0 = (v0.1 + 1f64) * HEIGHT as f64 / 2f64;
            let x1 = (v1.0 + 1f64) * WIDTH as f64 / 2f64;
            let y1 = (v1.1 + 1f64) * HEIGHT as f64 / 2f64;
            img.draw_line(x0 as u32, y0 as u32, x1 as u32, y1 as u32, Color::new(255, 255, 255));
        }
    }
    img.write_to_file("test.tga").unwrap();
}
