use std::{
    io::{self, Write},
    path::Path,
};

use crate::Pixels;

pub fn save_to_ppm_file<P>(pixels: &Pixels, file_path: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let file_path = file_path.as_ref();
    let mut file = std::fs::File::create(file_path)?;
    file.write_all(format!("P6\n{} {} 255\n", pixels.width(), pixels.height()).as_bytes())?;
    for pixel in pixels.pixels() {
        file.write_all(&[pixel.r(), pixel.g(), pixel.b()])?;
    }
    Ok(())
}
