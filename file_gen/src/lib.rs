use std::{
    io::{self, Write},
    path::Path,
};

use olive_rs::Canvas;

fn create_file<P>(file_path: P) -> io::Result<std::io::BufWriter<std::fs::File>>
where
    P: AsRef<Path>,
{
    let file_path = file_path.as_ref();
    let file = std::fs::File::create(file_path)?;
    let file = std::io::BufWriter::new(file);
    Ok(file)
}

pub fn save_to_ppm_file<P>(canvas: &Canvas, file_path: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let mut file = create_file(file_path)?;
    save_to_ppm_stream(canvas, &mut file)?;
    Ok(())
}

pub fn save_to_ppm_stream<S>(canvas: &Canvas, stream: &mut S) -> io::Result<()>
where
    S: Write,
{
    stream.write_all(format!("P6\n{} {} 255\n", canvas.width(), canvas.height()).as_bytes())?;
    for pixel in canvas.pixels() {
        let r = pixel.r() as usize * pixel.a() as usize / u8::MAX as usize;
        let g = pixel.g() as usize * pixel.a() as usize / u8::MAX as usize;
        let b = pixel.b() as usize * pixel.a() as usize / u8::MAX as usize;
        stream.write_all(&[r as u8, g as u8, b as u8])?;
    }
    Ok(())
}

pub fn save_to_png_file<P>(canvas: &Canvas, file_path: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let mut file = create_file(file_path)?;
    save_to_png_stream(canvas, &mut file)?;
    Ok(())
}

pub fn save_to_png_stream<S>(canvas: &Canvas, stream: &mut S) -> io::Result<()>
where
    S: Write,
{
    assert!(canvas.width() <= u32::MAX as usize);
    assert!(canvas.height() <= u32::MAX as usize);
    let mut encoder = png::Encoder::new(stream, canvas.width() as u32, canvas.height() as u32);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    let mut pixels = Vec::new();
    for pixel in canvas.pixels() {
        pixels.push(pixel.r());
        pixels.push(pixel.g());
        pixels.push(pixel.b());
        pixels.push(pixel.a());
    }
    writer.write_image_data(&pixels)?;
    Ok(())
}
