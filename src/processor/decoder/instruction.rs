use std::fmt::Display;

use anyhow::Result;

use crate::processor::{register::RegisterAlias, ProcessorError};

use super::{
    error::{InstructionDecodingError, InstructionDecodingErrorType},
    sign_extension,
};

pub trait RiscvInstruction {
    fn assembly(&self) -> String;
}

pub enum AluCode {
    Add,
    Sub,
    Slt,
    Sltu,
    Sll,
    Srl,
    Sra,
    Xor,
    Or,
    And,
}

impl RiscvInstruction for AluCode {
    fn assembly(&self) -> String {
        match self {
            AluCode::Add => "add",
            AluCode::Sub => "sub",
            AluCode::Slt => "slt",
            AluCode::Sltu => "sltu",
            AluCode::Sll => "sll",
            AluCode::Srl => "srl",
            AluCode::Sra => "sra",
            AluCode::Xor => "xor",
            AluCode::Or => "or",
            AluCode::And => "and",
        }
        .into()
    }
}

impl TryFrom<(u8, u8, bool)> for AluCode {
    type Error = ProcessorError;

    fn try_from((funct7, funct3, imm): (u8, u8, bool)) -> Result<Self, Self::Error> {
        let error_type = InstructionDecodingErrorType::InvalidAluOperation;
        let error = InstructionDecodingError::new(error_type);

        match funct3 {
            0b000 => {
                if imm {
                    Ok(AluCode::Add)
                } else {
                    match funct7 {
                        0b000000 => Ok(AluCode::Add),
                        0b010000 => Ok(AluCode::Sub),
                        _ => Err(error),
                    }
                }
            }
            0b001 => Ok(AluCode::Sll),
            0b010 => Ok(AluCode::Slt),
            0b011 => Ok(AluCode::Sltu),
            0b100 => Ok(AluCode::Xor),
            0b101 => match funct7 {
                0b000000 => Ok(AluCode::Srl),
                0b010000 => Ok(AluCode::Sra),
                _ => Err(error),
            },
            0b110 => Ok(AluCode::Or),
            0b111 => Ok(AluCode::And),
            _ => unreachable!(),
        }
    }
}

impl Display for AluCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.assembly())
    }
}

pub enum BranchOption {
    Equal,
    NotEqual,
    LessThan,
    GraterEqual,
    LessThanUnsigned,
    GraterEqualUnsigned,
}

impl RiscvInstruction for BranchOption {
    fn assembly(&self) -> String {
        match self {
            BranchOption::Equal => "beq",
            BranchOption::NotEqual => "bne",
            BranchOption::GraterEqual => "bge",
            BranchOption::GraterEqualUnsigned => "bgeu",
            BranchOption::LessThan => "blt",
            BranchOption::LessThanUnsigned => "bltu",
        }
        .into()
    }
}

impl TryFrom<u8> for BranchOption {
    type Error = ProcessorError;

    fn try_from(funct3: u8) -> Result<Self, Self::Error> {
        match funct3 {
            0b000 => Ok(BranchOption::Equal),
            0b001 => Ok(BranchOption::NotEqual),
            0b100 => Ok(BranchOption::LessThan),
            0b101 => Ok(BranchOption::GraterEqual),
            0b110 => Ok(BranchOption::LessThanUnsigned),
            0b111 => Ok(BranchOption::GraterEqualUnsigned),
            _ => {
                let error_type = InstructionDecodingErrorType::UndefinedBranchOption(funct3);
                Err(InstructionDecodingError::new(error_type))
            }
        }
    }
}

impl Display for BranchOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.assembly())
    }
}

pub enum ByteWideOption {
    Byte,
    HalfWord,
    Word,
    ByteUnsigned,
    HalfWordUnsigned,
}

impl ByteWideOption {
    pub fn trim(&self, raw: u32, diff: u32) -> u32 {
        match self {
            ByteWideOption::Byte => {
                let value = (raw >> (diff * 8)) % 2u32.pow(8);
                sign_extension(value, 7)
            }
            ByteWideOption::ByteUnsigned => (raw >> (diff * 8)) % 2u32.pow(8),
            ByteWideOption::HalfWord => {
                let value = (raw >> (diff * 8)) % 2u32.pow(16);
                sign_extension(value, 15)
            }
            ByteWideOption::HalfWordUnsigned => (raw >> (diff * 8)) % 2u32.pow(16),
            ByteWideOption::Word => raw,
        }
    }

    pub fn overwrite_mask(&self, diff: u32) -> u32 {
        match self {
            ByteWideOption::Byte | ByteWideOption::ByteUnsigned => (1 << (8 * diff)) * 0xFF,
            ByteWideOption::HalfWord | ByteWideOption::HalfWordUnsigned => {
                (1 << (8 * diff)) * 0xFFFF
            }
            ByteWideOption::Word => 0xFFFF_FFFF,
        }
    }
}

impl RiscvInstruction for ByteWideOption {
    fn assembly(&self) -> String {
        match self {
            ByteWideOption::Byte => "b",
            ByteWideOption::ByteUnsigned => "bu",
            ByteWideOption::HalfWord => "h",
            ByteWideOption::HalfWordUnsigned => "hu",
            ByteWideOption::Word => "w",
        }
        .into()
    }
}

impl TryFrom<u8> for ByteWideOption {
    type Error = ProcessorError;

    fn try_from(funct3: u8) -> Result<Self, Self::Error> {
        match funct3 {
            0b000 => Ok(ByteWideOption::Byte),
            0b001 => Ok(ByteWideOption::HalfWord),
            0b010 => Ok(ByteWideOption::Word),
            0b100 => Ok(ByteWideOption::ByteUnsigned),
            0b101 => Ok(ByteWideOption::HalfWordUnsigned),
            _ => {
                let error_type = InstructionDecodingErrorType::UndefinedByteWideOption(funct3);
                Err(InstructionDecodingError::new(error_type))
            }
        }
    }
}

impl Display for ByteWideOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.assembly())
    }
}

pub enum InstructionCode {
    Lui,
    Auipc,
    Jal,
    Jalr,
    Branch(BranchOption),
    Load(ByteWideOption),
    Store(ByteWideOption),
    Ope(AluCode),
    OpeI(AluCode),
}

impl TryFrom<(u8, u8, u8)> for InstructionCode {
    type Error = ProcessorError;

    fn try_from((funct7, funct3, opecode): (u8, u8, u8)) -> Result<Self, Self::Error> {
        match opecode {
            0x37 => Ok(InstructionCode::Lui),
            0x17 => Ok(InstructionCode::Auipc),
            0x6F => Ok(InstructionCode::Jal),
            0x67 => Ok(InstructionCode::Jalr),
            0x63 => {
                // BRANCH
                let branch_option = BranchOption::try_from(funct3)?;
                Ok(InstructionCode::Branch(branch_option))
            }
            0x03 => {
                // LOAD
                let bytewide_option = ByteWideOption::try_from(funct3)?;
                Ok(InstructionCode::Load(bytewide_option))
            }
            0x23 => {
                // STORE
                let bytewide_option = ByteWideOption::try_from(funct3)?;
                match bytewide_option {
                    ByteWideOption::ByteUnsigned | ByteWideOption::HalfWordUnsigned => {
                        let error_type = InstructionDecodingErrorType::StoreMustBeSigned;
                        let error = InstructionDecodingError::new(error_type);
                        Err(error)
                    }
                    _ => Ok(InstructionCode::Store(bytewide_option)),
                }
            }
            0x13 => {
                // OpeI
                let code = AluCode::try_from((funct7, funct3, true))?;
                Ok(InstructionCode::OpeI(code))
            }
            0x33 => {
                // Ope
                let code = AluCode::try_from((funct7, funct3, false))?;
                Ok(InstructionCode::Ope(code))
            }
            0x0F => {
                // fence
                unimplemented!()
            }
            0x73 => {
                // csrr
                unimplemented!()
            }
            _ => unimplemented!(),
        }
    }
}

pub enum RiscvForm {
    R,
    I,
    B,
    S,
    J,
    U,
}

impl RiscvForm {}

impl TryFrom<u8> for RiscvForm {
    type Error = ProcessorError;

    fn try_from(opecode: u8) -> Result<Self, Self::Error> {
        match opecode {
            51 => Ok(RiscvForm::R),
            3 | 19 | 103 => Ok(RiscvForm::I),
            99 => Ok(RiscvForm::B),
            35 => Ok(RiscvForm::S),
            111 => Ok(RiscvForm::J),
            23 | 55 => Ok(RiscvForm::U),
            _ => {
                let error_type = InstructionDecodingErrorType::UndefinedRiscvForm;
                Err(InstructionDecodingError::new(error_type))
            }
        }
    }
}

pub struct Instruction {
    _inst: u32,
    pub code: InstructionCode,
    pub form: RiscvForm,
    pub rs1: u8,
    pub rs2: u8,
    pub rd: u8,
    pub imm: u32,
    pub is_halt: bool,
}

impl Instruction {
    pub fn new(
        instruction: u32,
        code: InstructionCode,
        form: RiscvForm,
        registers: [u8; 3],
        imm: u32,
    ) -> Self {
        let is_halt = instruction == 0;

        Self {
            _inst: instruction,
            code,
            form,
            rs1: registers[0],
            rs2: registers[1],
            rd: registers[2],
            imm,
            is_halt,
        }
    }
}

impl RiscvInstruction for Instruction {
    fn assembly(&self) -> String {
        let rs1 = RegisterAlias::try_from(self.rs1).ok().unwrap();
        let rs2 = RegisterAlias::try_from(self.rs2).ok().unwrap();
        let rd = RegisterAlias::try_from(self.rd).ok().unwrap();
        let imm = self.imm as i32;

        match &self.code {
            InstructionCode::Auipc => {
                format!("auipc {}, {}", rd, imm)
            }
            InstructionCode::Branch(opt) => {
                format!("{} {}, {}, {}", opt, rs1, rs2, imm)
            }
            InstructionCode::Jal => {
                format!("jal {}, {}", rd, imm)
            }
            InstructionCode::Jalr => {
                format!("jalr {}, {}, {}", rd, rs1, imm)
            }
            InstructionCode::Load(opt) => {
                format!("l{} {}, {}, ({})", opt, rd, imm, rs1)
            }
            InstructionCode::Store(opt) => {
                format!("s{} {}, {}, ({})", opt, rs2, imm, rs1)
            }
            InstructionCode::Lui => {
                format!("lui {}, {}", rd, imm)
            }
            InstructionCode::Ope(ope) => {
                format!("{} {}, {}, {}", ope, rd, rs1, rs2)
            }
            InstructionCode::OpeI(ope) => {
                format!("{}i {}, {}, {}", ope, rd, rs1, imm)
            }
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}; {:#010x}", self.assembly(), self._inst)
    }
}

#[cfg(test)]
pub mod tests {
    use super::ByteWideOption;

    #[test]
    pub fn test_byte_wide_option() {
        let opt = ByteWideOption::Byte;
        assert_eq!(opt.overwrite_mask(0), 0x000000FF);
        assert_eq!(opt.overwrite_mask(1), 0x0000FF00);
        assert_eq!(opt.overwrite_mask(2), 0x00FF0000);
        assert_eq!(opt.overwrite_mask(3), 0xFF000000);
        let opt = ByteWideOption::HalfWord;
        assert_eq!(opt.overwrite_mask(0), 0x0000FFFF);
        assert_eq!(opt.overwrite_mask(1), 0x00FFFF00);
        assert_eq!(opt.overwrite_mask(2), 0xFFFF0000);
        assert_eq!(!opt.overwrite_mask(2), 0x0000FFFF);
        let opt = ByteWideOption::Word;
        assert_eq!(opt.overwrite_mask(0), 0xFFFFFFFF);
    }
}
