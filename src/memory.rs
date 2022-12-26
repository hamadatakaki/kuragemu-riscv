use std::fmt::Display;

use anyhow::Result;

use crate::processor::{decoder::instruction::ByteWideOption, ProcessorError, ProcessorErrorTrait};

const DATA_MEMORY_SIZE: usize = 0x10000;

#[derive(Clone)]
pub struct MainMemory {
    mem: [u32; DATA_MEMORY_SIZE],
}

impl MainMemory {
    pub fn new() -> Self {
        Self {
            mem: [0; DATA_MEMORY_SIZE],
        }
    }

    pub fn read(&self, address: u32, option: &ByteWideOption) -> Result<u32, ProcessorError> {
        let addrdiff = address % 4;
        let address = address >> 2;

        if address < DATA_MEMORY_SIZE as u32 {
            let raw = self.mem[address as usize];
            let rd = option.trim(raw, addrdiff);
            Ok(rd)
        } else {
            let error_type = MainMemoryErrorType::AddressOutOfBounds;
            Err(MainMemoryError::new(error_type))
        }
    }

    pub fn write(
        &mut self,
        address: u32,
        raw: u32,
        option: &ByteWideOption,
    ) -> Result<(), ProcessorError> {
        let addrdiff = address % 4;
        let address = address >> 2;

        if address < DATA_MEMORY_SIZE as u32 {
            let mask = option.overwrite_mask(addrdiff);
            let value = (raw << (addrdiff * 8)) & mask;
            self.mem[address as usize] &= !mask;
            self.mem[address as usize] |= value;
            Ok(())
        } else {
            let error_type = MainMemoryErrorType::AddressOutOfBounds;
            Err(MainMemoryError::new(error_type))
        }
    }

    pub fn head(&self, n: usize) -> Vec<u32> {
        let view = &self.mem[0..n];
        view.to_vec()
    }
}

pub enum MainMemoryErrorType {
    AddressOutOfBounds,
}

impl Display for MainMemoryErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AddressOutOfBounds => {
                write!(f, "violate address bounds by memory access")
            }
        }
    }
}

pub struct MainMemoryError {
    error_type: MainMemoryErrorType,
}

impl MainMemoryError {
    fn new(error_type: MainMemoryErrorType) -> Box<Self> {
        let error = Self { error_type };
        Box::new(error)
    }
}

impl Display for MainMemoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.form())
    }
}

impl ProcessorErrorTrait for MainMemoryError {
    fn form(&self) -> String {
        format!("memory access is failed - {}", self.error_type)
    }
}
