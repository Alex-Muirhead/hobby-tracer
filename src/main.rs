use std::fs::File;
use std::io::{Error, Write};

#[derive(Debug)]
struct PGM {
    width: usize,
    height: usize,
    max_value: u8,
    data: Vec<u8>,
}

impl PGM {
    fn new(width: usize, height: usize, max_value: u8) -> Self {
        let data = vec![0; width * height];
        PGM {
            width,
            height,
            max_value,
            data,
        }
    }

    fn write(&self, output: &mut File) -> Result<(), Error> {
        // Header
        write!(
            output,
            "P2\n{} {}\n{}\n",
            self.width, self.height, self.max_value
        )?;
        // Data
        for idx in 0..(self.width * self.height) {
            write!(output, "{} ", self.data[idx])?;
        }

        Ok(())
    }
}

fn main() -> Result<(), Error> {
    // --- No libraries at first! --

    // Let's write a simple PBM file
    let width = 9;
    let height = 16;
    let max_value = 255;

    let filename = "render.pgm";
    let mut output = File::create(filename)?;

    let mut image = PGM::new(width, height, max_value);

    // Some fake data to help me get started
    for row in 0..height {
        for col in 0..width {
            let idx = row * width + col;
            // Should hopefully make a gradient pattern
            image.data[idx] = (idx % 2 * row) as u8;
        }
    }

    // Contents
    image.write(&mut output)?;

    Ok(())
}
