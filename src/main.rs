use std::fs::File;
use std::io::{Error, Write};

fn main() -> Result<(), Error> {
    // --- No libraries at first! --

    // Let's write a simple PBM file
    let width = 9;
    let height = 16;

    let filename = "render.pbm";
    let mut output = File::create(filename)?;

    // Header
    writeln!(output, "P1")?;
    writeln!(output, "{width} {height}")?;

    // Some fake data to help me get started
    let mut data = vec![0; width * height];
    for row in 0..height {
        for col in 0..width {
            let idx = row * width + col;
            // Should hopefully make a checkerboard pattern
            data[idx] = idx % 2;
        }
    }

    // Contents
    for row in 0..height {
        for col in 0..width {
            let idx = row * width + col;
            write!(output, "{} ", data[idx])?;
        }
        // Not _entirely_ necessary
        write!(output, "\n")?;
    }

    Ok(())
}
