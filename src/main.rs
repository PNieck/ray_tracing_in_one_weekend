use std::process;

use ray_tracing_in_one_weekend as ray_tracing;

fn main() {
    let img_width = 256;
    let aspect_ratio = 16.0/9.0;
    
    if let Err(e) = ray_tracing::render_image("result.ppm", img_width, aspect_ratio) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
