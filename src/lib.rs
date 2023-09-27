use std::{fs::File, io::{Write, BufWriter, self}, error::Error};


pub fn render_image(file_path: &str, img_height: i32, img_width: i32) -> Result<(), Box<dyn Error>> {
    let max_color = 255;

    let file = File::create(file_path)?;
    let mut writer = BufWriter::new(file);

    println!("Rendering started...");

    writer.write_all("P3\n".as_bytes())?;
    writer.write_all(format!("{img_width} {img_height}\n").as_bytes())?;
    writer.write_all(format!("{max_color}\n").as_bytes())?;

    for height in 0..img_height {
        print!("\rRemaining scanlines: {:>5}", img_height - height - 1);
        io::stdout().flush().expect("Cannot flush stdout");

        for row in 0..img_width {
            let r = row % (max_color + 1);
            let g = height % (max_color + 1);
            let b = 0;

            writer.write_all(format!("{r} {g} {b}\n").as_bytes())?;
        }
    }

    println!("\rDone!                     ");

    return Ok(());
}
