use std::{fs::File, io::{Write, BufWriter, self}};

fn write_to_file(writer: &mut BufWriter<File>, text: &str) {
    writer.write_all(text.as_bytes())
        .expect("Cannot write to file");
}

fn main() {
    let image_width = 256;
    let image_height = 256;
    let max_color = 255;

    let file = File::create("result.ppm").expect("Cannot create a file");
    let mut writer = BufWriter::new(file);

    println!("Rendering started...");

    write_to_file(&mut writer, "P3\n");
    write_to_file(&mut writer, format!("{image_width} {image_height}\n").as_str());
    write_to_file(&mut writer, format!("{max_color}\n").as_str());

    for height in 0..image_height {
        print!("\rRemaining scanlines: {:>5}", image_height - height - 1);
        io::stdout().flush().expect("Cannot flush stdout");

        for row in 0..image_width {
            let r = row % (max_color + 1);
            let g = height % (max_color + 1);
            let b = 0;

            write_to_file(&mut writer, format!("{r} {g} {b}\n").as_str());
        }
    }

    println!("\rDone!                     ")
}
