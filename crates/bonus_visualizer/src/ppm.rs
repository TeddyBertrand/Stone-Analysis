// crates/stone_analysis/src/visualizer/ppm.rs

use super::color::Rgb;
use audio::errors::AudioError;
use std::fs::File;
use std::io::{BufWriter, Write};

pub struct PpmWriter {
    pub pixels: Vec<Rgb>,
    pub width: usize,
    pub height: usize,
}

impl PpmWriter {
    pub fn new(width: usize, height: usize, bg: Rgb) -> Self {
        Self {
            pixels: vec![bg; width * height],
            width,
            height,
        }
    }

    #[inline]
    pub fn set(&mut self, x: usize, y: usize, color: Rgb) {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x] = color;
        }
    }

    pub fn save(&self, path: &str) -> Result<(), AudioError> {
        let file = File::create(path).map_err(|_| AudioError::EmptySignal)?;
        let mut w = BufWriter::new(file);
        write!(w, "P6\n{} {}\n255\n", self.width, self.height).unwrap();
        for Rgb(r, g, b) in &self.pixels {
            w.write_all(&[*r, *g, *b]).unwrap();
        }
        w.flush().unwrap();
        Ok(())
    }
}
