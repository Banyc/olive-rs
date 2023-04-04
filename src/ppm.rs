use std::{
    io::{self, Write},
    path::Path,
};

use crate::Canvas;

pub fn save_to_ppm_file<P>(canvas: &Canvas, file_path: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let file_path = file_path.as_ref();
    let mut file = std::fs::File::create(file_path)?;
    save_to_ppm_stream(canvas, &mut file)?;
    Ok(())
}

pub fn save_to_ppm_stream<S>(canvas: &Canvas, stream: &mut S) -> io::Result<()>
where
    S: Write,
{
    stream.write_all(format!("P6\n{} {} 255\n", canvas.width(), canvas.height()).as_bytes())?;
    for pixel in canvas.pixels() {
        stream.write_all(&[pixel.r(), pixel.g(), pixel.b()])?;
    }
    Ok(())
}
