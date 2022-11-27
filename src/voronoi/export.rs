use std::fs::File;
use std::io::Write;
use byteorder::ByteOrder;

pub fn export_image(file_name: &str, map: Vec<Vec<u32>>, width: usize, height: usize) {
    let mut file = File::create(file_name).expect("Unable to create a file");

    // Write the header of the file
    file.write(b"P6 ").expect("Unable to write to file");
    file.write_fmt(format_args!("{} ", width)).expect("Unable to write width to file");
    file.write_fmt(format_args!("{} ", height)).expect("Unable to write height to file");
    file.write(b"255\n").expect("Unable to write color to file");

    // Write the map to the file
    // 0x00BBGGRR
    let mut current_color: [u8; 4] = [0,0,0,0];
    for i in 0..width {
        for j in 0..height {
            byteorder::NativeEndian::write_u32(&mut current_color[0..4], map[i][j]);
            file.write(&[current_color[0]]).expect("Unable to write color");
            file.write(&[current_color[1]]).expect("Unable to write color");
            file.write(&[current_color[2]]).expect("Unable to write color");
        }
    }
}