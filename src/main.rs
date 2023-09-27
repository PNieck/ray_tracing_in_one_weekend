use std::process;

use ray_tracing_in_one_weekend as ray_tracing;

fn main() {
    let image_width = 256;
    let image_height = 256;
    
    if let Err(e) = ray_tracing::render_image("result.ppm", image_height, image_width) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
