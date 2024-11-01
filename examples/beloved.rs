use master_render_3d::{ansi::{ANSIRenderer, ANSIStyle}, render_math::{matrix::Mat4, vector::{Mesh, Vector3, Vector2i}}};

fn main() {
    let mut renderer = ANSIRenderer::new();

    let mut heart = Mesh::new();
    // Create heart mesh
    // Left half
    heart.add_vertex(Vector3::new(0.0, 0.0, 0.0));
    heart.add_vertex(Vector3::new(-0.66497, 0.3544, 0.0));
    heart.add_vertex(Vector3::new(-0.895847, 0.59067, 0.0));
    heart.add_vertex(Vector3::new(-1.142, 0.82694, 0.0));
    heart.add_vertex(Vector3::new(-1.35854, 1.1387, 0.0));
    heart.add_vertex(Vector3::new(-1.5751, 1.4504, 0.0));
    heart.add_vertex(Vector3::new(-1.6834, 1.71294, 0.0));
    heart.add_vertex(Vector3::new(-1.7917, 1.9755, 0.0));
    heart.add_vertex(Vector3::new(-1.8475, 2.4644, 0.0));
    heart.add_vertex(Vector3::new(-1.7277, 2.8762, 0.0));
    heart.add_vertex(Vector3::new(-1.516, 3.1699, 0.0));
    heart.add_vertex(Vector3::new(-1.2273, 3.3012, 0.0));
    heart.add_vertex(Vector3::new(-0.92538, 3.3537, 0.0));
    heart.add_vertex(Vector3::new(-0.51848, 3.3209, 0.0));
    heart.add_vertex(Vector3::new(-0.25267, 3.2159, 0.0));
    heart.add_vertex(Vector3::new(0.0, 2.9796, 0.0));

    // Right half (mirrored)
    heart.add_vertex(Vector3::new(0.0, 2.9796, 0.0));
    heart.add_vertex(Vector3::new(0.25267, 3.2159, 0.0));
    heart.add_vertex(Vector3::new(0.51848, 3.3209, 0.0));
    heart.add_vertex(Vector3::new(0.92538, 3.3537, 0.0));
    heart.add_vertex(Vector3::new(1.2273, 3.3012, 0.0));
    heart.add_vertex(Vector3::new(1.516, 3.1699, 0.0));
    heart.add_vertex(Vector3::new(1.7277, 2.8762, 0.0));
    heart.add_vertex(Vector3::new(1.8475, 2.4644, 0.0));
    heart.add_vertex(Vector3::new(1.7917, 1.9755, 0.0));
    heart.add_vertex(Vector3::new(1.6834, 1.71294, 0.0));
    heart.add_vertex(Vector3::new(1.5751, 1.4504, 0.0));
    heart.add_vertex(Vector3::new(1.35854, 1.1387, 0.0));
    heart.add_vertex(Vector3::new(1.142, 0.82694, 0.0));
    heart.add_vertex(Vector3::new(0.895847, 0.59067, 0.0));
    heart.add_vertex(Vector3::new(0.66497, 0.3544, 0.0));
    heart.add_vertex(Vector3::new(0.324868, 0.1772, 0.0));

    let default_heart_scale = Vector3::new(20.0, -10.0, 1.0);
    let scale_matrix = Mat4::scale(default_heart_scale);
    let translation_matrix = Mat4::translation(Vector3::new(55.0, 40.0, 0.0));

    heart.set_scale_matrix(scale_matrix);
    heart.set_translation_matrix(translation_matrix);

    // LoveYou image
    let image = image_helper::image::read_image_from_file("LoveYou.png").expect("Tech demo segment.");

    let mut scale_factor = 1.0;
    let start_time = std::time::Instant::now();
    let mut time = start_time;
    let mut delta: f64;
    let mut actual_delta: f64;
    let mut delta_span: std::time::Duration;
    let mut total_span: std::time::Duration;
    let mut total_millis: f64;

    let mut last_beat = 0;

    loop {
        delta_span = std::time::Instant::now() - time;
        delta = f64::from(delta_span.as_millis() as u32);
        total_span = std::time::Instant::now() - start_time;
        total_millis = f64::from(total_span.as_millis() as u32);

        actual_delta = delta;
        if delta < 16.6 {
            std::thread::sleep(
                std::time::Duration::from_millis( (16.6 - delta) as u64 )
            );
            delta = 16.6;
        }

        let heart_offset_x = (total_millis / 1500.0).sin() * 2.0;
        let translation_matrix = Mat4::translation(Vector3::new(55.0 + heart_offset_x, 40.0, 0.0));
        heart.set_translation_matrix(translation_matrix);

        if scale_factor > 1.0 {
            scale_factor -= delta / 500.0
        }
        else {
            scale_factor = 1.0;
        }

        let beat = (total_millis / 500.0) as u32;
        if beat != last_beat {
            last_beat = beat;
            scale_factor = 1.05;
        }

        heart.set_scale_matrix(Mat4::scale(default_heart_scale * scale_factor));

        let rotation_matrix = Mat4::euler_rotation(Vector3::new(
            (total_millis / 4200.0).sin() * 0.2,
            total_millis / 5000.0,
            (total_millis / 3800.0).sin() * 0.2));
        heart.set_rotation_matrix(rotation_matrix);

        let render_start = std::time::Instant::now();
        _ = renderer.rasterize_vertices(&heart, 3.0);
        _ = renderer.draw_image_2d(&image, Vector2i::new(28 + heart_offset_x as i32 + 2, 10), 3.0);
        _ = renderer.draw_at(0, 0, &actual_delta.to_string(), 0.0, Some(ANSIStyle::Underline));
        renderer.set_style(ANSIStyle::None);
        _ = renderer.flush();
        
        _ = renderer.draw_at(1, 2, &(std::time::Instant::now() - render_start).as_nanos().to_string(), 0.0, None); // absolute 0 after rendering refactor of 31.10.2024? Suspicious...

        time = std::time::Instant::now();
    }
}