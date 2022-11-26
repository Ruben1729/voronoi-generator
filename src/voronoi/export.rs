use std::fs::File;
use std::io::Write;
use byteorder::ByteOrder;

pub fn export_image(file_name: &str, map: Vec<u32>, width: usize, height: usize) {
    let mut file = File::create(file_name).expect("Unable to create a file");

    // Write the header of the file
    file.write(b"P6 ").expect("Unable to write to file");
    file.write_fmt(format_args!("{} ", width)).expect("Unable to write width to file");
    file.write_fmt(format_args!("{} ", height)).expect("Unable to write height to file");
    file.write(b"255 ").expect("Unable to write color to file");
    file.write(b"\n").expect("Unable to write new line");

    // Write the map to the file
    // 0x00BBGGRR
    let mut current_color: [u8; 4] = [0,0,0,0];
    for pixel in map {
        byteorder::NativeEndian::write_u32(&mut current_color[0..4], pixel);
        println!("Color: {} {} {} {}", current_color[0], current_color[1], current_color[2], current_color[3]);
        file.write_fmt(format_args!("{} ", current_color[1])).expect("Unable to write red value");
        file.write_fmt(format_args!("{} ", current_color[2])).expect("Unable to write red value");
        file.write_fmt(format_args!("{} ", current_color[3])).expect("Unable to write red value");
    }
}