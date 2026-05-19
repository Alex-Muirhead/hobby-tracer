#![allow(dead_code)]
use std::fs::File;
use std::io::{Error, Write};

#[derive(Debug)]
pub struct PGM {
    width: usize,
    height: usize,
    max_value: u8,
    pub data: Vec<u8>,
}

impl PGM {
    pub fn new(width: usize, height: usize, max_value: u8) -> Self {
        let data = vec![0; width * height];
        PGM {
            width,
            height,
            max_value,
            data,
        }
    }

    pub fn write(&self, output: &mut File) -> Result<(), Error> {
        // Header
        write!(
            output,
            "P2\n{} {}\n{}\n",
            self.width, self.height, self.max_value
        )?;
        // Data
        for idx in 0..(self.width * self.height) {
            write!(output, "{}\n", self.data[idx])?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct PPM {
    width: usize,
    height: usize,
    max_value: u8,
    pub data: Vec<(u8, u8, u8)>,
}

impl PPM {
    pub fn new(width: usize, height: usize, max_value: u8) -> Self {
        let data = vec![(0, 0, 0); width * height];
        PPM {
            width,
            height,
            max_value,
            data,
        }
    }

    pub fn write(&self, output: &mut impl Write) -> Result<(), Error> {
        // Header
        write!(
            output,
            "P3\n{} {}\n{}\n",
            self.width, self.height, self.max_value
        )?;
        // Data
        for idx in 0..(self.width * self.height) {
            let pixel = self.data[idx];
            write!(output, "{} {} {}\n", pixel.0, pixel.1, pixel.2)?;
        }

        Ok(())
    }
}
