pub trait Pixels2D {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn pixels(&self) -> &[Pixel];
    fn pixels_mut(&mut self) -> &mut [Pixel];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StackPixels2D<const N: usize> {
    pixels: [Pixel; N],
    width: usize,
    height: usize,
}

impl<const N: usize> StackPixels2D<N> {
    pub fn new(width: usize, height: usize, fill: Pixel) -> StackPixels2D<N> {
        assert_eq!(width * height, N);
        StackPixels2D {
            pixels: [fill; N],
            width,
            height,
        }
    }
}

impl<const N: usize> Pixels2D for StackPixels2D<N> {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn pixels(&self) -> &[Pixel] {
        &self.pixels
    }

    fn pixels_mut(&mut self) -> &mut [Pixel] {
        &mut self.pixels
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeapPixels2D {
    pixels: Vec<Pixel>,
    width: usize,
    height: usize,
}

impl HeapPixels2D {
    pub fn new(width: usize, height: usize, fill: Pixel) -> HeapPixels2D {
        HeapPixels2D {
            pixels: vec![fill; width * height],
            width,
            height,
        }
    }
}

impl Pixels2D for HeapPixels2D {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn pixels(&self) -> &[Pixel] {
        &self.pixels
    }

    fn pixels_mut(&mut self) -> &mut [Pixel] {
        &mut self.pixels
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Pixel {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Pixel {
        Pixel { r, g, b, a }
    }

    pub fn r(&self) -> u8 {
        self.r
    }

    pub fn g(&self) -> u8 {
        self.g
    }

    pub fn b(&self) -> u8 {
        self.b
    }

    pub fn a(&self) -> u8 {
        self.a
    }

    #[must_use]
    pub fn over(&self, other: Pixel) -> Pixel {
        /// - `c1` over `c2`
        /// - Ref: <https://en.wikipedia.org/wiki/Alpha_compositing>
        fn mix_colors(c1: Pixel, c2: Pixel) -> Pixel {
            let a1 = c1.a as f64 / u8::MAX as f64;
            let a2 = c2.a as f64 / u8::MAX as f64;
            let a0 = a1 + (1. - a1) * a2;
            let r = mix_comps(c1.r, c2.r, a0, a1, a2);
            let g = mix_comps(c1.g, c2.g, a0, a1, a2);
            let b = mix_comps(c1.b, c2.b, a0, a1, a2);
            let a = mix_comps(c1.a, c2.a, a0, a1, a2);
            Pixel::new(r, g, b, a)
        }

        /// `c1` over `c2`
        fn mix_comps(c1: u8, c2: u8, a0: f64, a1: f64, a2: f64) -> u8 {
            if a0 == 0. {
                return 0;
            }
            let c1 = c1 as f64;
            let c2 = c2 as f64;
            let c0 = (c1 * a1 + (1. - a1) * a2 * c2) / a0;
            c0 as u8
        }

        mix_colors(*self, other)
    }

    /// AABBGGRR
    pub fn to_u32(&self) -> u32 {
        (self.a as u32) << 0x18 | (self.b as u32) << 0x10 | (self.g as u32) << 0x8 | (self.r as u32)
    }
}

impl From<u32> for Pixel {
    fn from(value: u32) -> Pixel {
        Pixel {
            r: (value & 0x000000FF) as u8,
            g: ((value & 0x0000FF00) >> 0x8) as u8,
            b: ((value & 0x00FF0000) >> 0x10) as u8,
            a: ((value & 0xFF000000) >> 0x18) as u8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pixel() {
        let raw = 0x44332211;
        let pixel = Pixel::from(raw);
        assert_eq!(pixel.r(), 0x11);
        assert_eq!(pixel.g(), 0x22);
        assert_eq!(pixel.b(), 0x33);
        assert_eq!(pixel.a(), 0x44);
        assert_eq!(pixel.to_u32(), 0x44332211);
    }
}
