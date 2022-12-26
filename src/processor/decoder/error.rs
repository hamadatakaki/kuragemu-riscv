use std::fmt::Display;

use crate::processor::ProcessorErrorTrait;

pub enum InstructionDecodingErrorType {
    UndefinedBranchOption(u8),
    UndefinedByteWideOption(u8),
    UndefinedRiscvForm,
    StoreMustBeSigned,
    InvalidAluOperation,
}

impl Display for InstructionDecodingErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UndefinedBranchOption(opt) => {
                write!(f, "get undefined branch option: {}", opt)
            }
            Self::UndefinedByteWideOption(opt) => {
                write!(f, "get undefined byte-wide option: {}", opt)
            }
            Self::UndefinedRiscvForm => {
                write!(f, "get undefined riscv form")
            }
            Self::StoreMustBeSigned => {
                write!(f, "load instruction must be signed")
            }
            Self::InvalidAluOperation => {
                write!(f, "invalid (funct7, funct3) value given")
            }
        }
    }
}

pub struct InstructionDecodingError {
    error_type: InstructionDecodingErrorType,
}

impl InstructionDecodingError {
    pub fn new(error_type: InstructionDecodingErrorType) -> Box<Self> {
        let error = Self { error_type };
        Box::new(error)
    }
}

impl Display for InstructionDecodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.form())
    }
}

impl ProcessorErrorTrait for InstructionDecodingError {
    fn form(&self) -> String {
        format!("decode instruction is failed - {}", self.error_type)
    }
}
