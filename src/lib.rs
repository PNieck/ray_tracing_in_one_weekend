use std::{fs::File, io::{Write, BufWriter, self}, error::Error, cmp};

use primitives::{color::{self, Color}, ray::Ray, position::Position, vec3::Vec3};

mod primitives;

pub fn render_image(file_path: &str, img_width: i32, aspect_ratio: f64) -> Result<(), Box<dyn Error>> {
    if aspect_ratio <= 0.0 {
        return Err(Box::<dyn Error>::from("Invalid argument: aspect_ratio must be positive")) 
    }

    // Calculate the image height, and ensure that it's at least 1.
    let img_height = cmp::max((img_width as f64 / aspect_ratio) as i32, 1);
    
    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * img_width as f64 / img_height as f64;
    let camera_center = Position::zero();

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_h = Vec3::unit_x() * viewport_width;
    let viewport_vd = Vec3::unit_y() * -viewport_height;

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_h = viewport_h / img_width as f64;
    let pixel_delta_v = viewport_vd / img_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_center = Position::translation(camera_center, Vec3::new(0.0, 0.0, -focal_length));
    let viewport_upper_left = Position::translation(viewport_center, -viewport_h/2 - viewport_vd/2);

    let pixel00 = Position::translation(viewport_upper_left, (pixel_delta_h + pixel_delta_v)/2.0);

    // Getting a file writer
    let file = File::create(file_path)?;
    let mut writer = BufWriter::new(file);

    println!("Rendering started...");

    writer.write_all("P3\n".as_bytes())?;
    writer.write_all(format!("{img_width} {img_height}\n").as_bytes())?;
    writer.write_all(format!("{}\n", color::MAX_VALUE).as_bytes())?;

    for column in 0..img_height {
        print!("\rRemaining scanlines: {:>5}", img_height - column - 1);
        io::stdout().flush().expect("Cannot flush stdout");

        for row in 0..img_width {
            let pixel_center = Position::translation(pixel00, column * pixel_delta_h + row * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&ray);
            save_pixel(pixel_color, &mut writer)?;
        }
    }

    println!("\rDone!                     ");

    return Ok(());
}


fn ray_color(_ray: &Ray) -> Color {
    Color::from_rgb(0, 0, 0)
}


fn save_pixel(color: Color, writer: &mut BufWriter<File>) -> io::Result<()> {
    writer.write_all(format!("{} {} {}\n", color.red(), color.green(), color.blue()).as_bytes())
}
