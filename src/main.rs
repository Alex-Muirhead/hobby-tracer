use std::fs::File;
use std::io::{Error, Write};

fn main() -> Result<(), Error> {
    // --- No libraries at first! --

    // Let's write a simple PBM file
    let width = 9;
    let height = 16;
    let max_value = 15;

    let filename = "render.pgm";
    let mut output = File::create(filename)?;

    // Header
    writeln!(output, "P2")?;
    writeln!(output, "{width} {height}")?;
    writeln!(output, "{max_value}")?;

    // Some fake data to help me get started
    let mut data = vec![0; width * height];
    for row in 0..height {
        for col in 0..width {
            let idx = row * width + col;
            // Should hopefully make a checkerboard pattern
            data[idx] = idx % 2 * row;
        }
    }

    // Contents
    for row in 0..height {
        for col in 0..width {
            let idx = row * width + col;
            write!(output, "{} ", data[idx])?;
        }
    }

    Ok(())
}
