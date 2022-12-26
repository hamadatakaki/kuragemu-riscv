use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::Result;

pub const INSTRUCTION_MEMORY_SIZE: usize = 0x10000;
pub const INSTRUCTION_MEMORY_INIT: usize = 0x2000;

pub struct Fetcher {
    pub pc: u32,
    mem: [u32; INSTRUCTION_MEMORY_SIZE],
}

impl Fetcher {
    pub fn new() -> Self {
        Self {
            pc: INSTRUCTION_MEMORY_INIT as u32,
            mem: [0; INSTRUCTION_MEMORY_SIZE],
        }
    }

    pub fn update_program_counter(&mut self, pc: u32) {
        self.pc = pc;
    }

    pub fn fetch(&self) -> u32 {
        let index = self.pc >> 2;
        self.mem[index as usize]
    }

    pub fn load_hex<P>(&mut self, path: P, diff: usize) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut index = diff >> 2;

        for line in reader.lines() {
            self.mem[index] = u32::from_str_radix(line?.as_str(), 16)?;
            index += 1;
        }

        Ok(())
    }
}
