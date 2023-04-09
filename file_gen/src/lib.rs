use std::{
    io::{self, Write},
    path::Path,
};

use olive_rs::Pixels2D;

fn create_file<P>(file_path: P) -> io::Result<std::io::BufWriter<std::fs::File>>
where
    P: AsRef<Path>,
{
    let file_path = file_path.as_ref();
    let file = std::fs::File::create(file_path)?;
    let file = std::io::BufWriter::new(file);
    Ok(file)
}

pub fn save_to_ppm_file<P, CP>(pixels: &CP, file_path: P) -> io::Result<()>
where
    P: AsRef<Path>,
    CP: Pixels2D,
{
    let mut file = create_file(file_path)?;
    save_to_ppm_stream(pixels, &mut file)?;
    Ok(())
}

pub fn save_to_ppm_stream<S, CP>(pixels: &CP, stream: &mut S) -> io::Result<()>
where
    S: Write,
    CP: Pixels2D,
{
    stream.write_all(format!("P6\n{} {} 255\n", pixels.width(), pixels.height()).as_bytes())?;
    for pixel in pixels.pixels() {
        let r = pixel.r() as usize * pixel.a() as usize / u8::MAX as usize;
        let g = pixel.g() as usize * pixel.a() as usize / u8::MAX as usize;
        let b = pixel.b() as usize * pixel.a() as usize / u8::MAX as usize;
        stream.write_all(&[r as u8, g as u8, b as u8])?;
    }
    Ok(())
}

pub fn save_to_png_file<P, CP>(pixels: &CP, file_path: P) -> io::Result<()>
where
    P: AsRef<Path>,
    CP: Pixels2D,
{
    let mut file = create_file(file_path)?;
    save_to_png_stream(pixels, &mut file)?;
    Ok(())
}

pub fn save_to_png_stream<S, CP>(pixels: &CP, stream: &mut S) -> io::Result<()>
where
    S: Write,
    CP: Pixels2D,
{
    assert!(pixels.width() <= u32::MAX as usize);
    assert!(pixels.height() <= u32::MAX as usize);
    let mut encoder = png::Encoder::new(stream, pixels.width() as u32, pixels.height() as u32);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    let mut png_pixels = Vec::new();
    for pixel in pixels.pixels() {
        png_pixels.push(pixel.r());
        png_pixels.push(pixel.g());
        png_pixels.push(pixel.b());
        png_pixels.push(pixel.a());
    }
    writer.write_image_data(&png_pixels)?;
    Ok(())
}
