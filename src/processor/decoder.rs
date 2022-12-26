pub mod error;
pub mod instruction;

use anyhow::Result;

use instruction::{Instruction, InstructionCode, RiscvForm};

use super::ProcessorError;

pub fn sign_extension(value: u32, n_top: u32) -> u32 {
    let sign = (value >> n_top) % 2;
    let mut value = value % 2u32.pow(n_top);
    if sign == 1 {
        value = u32::MAX - 2u32.pow(n_top) + value + 1
    }
    value
}

fn immediate(instruction: u32, form: &RiscvForm) -> u32 {
    let funct3 = ((instruction >> 12) % 8) as u8;

    match form {
        RiscvForm::I => match funct3 {
            0b001 | 0b101 => sign_extension(instruction >> 20, 4),
            _ => sign_extension(instruction >> 20, 11),
        },
        RiscvForm::S => sign_extension((instruction >> 25) * 32 + (instruction >> 7) % 32, 11),
        RiscvForm::B => {
            let imm11 = (instruction >> 7) % 2;
            let imm41 = (instruction >> 8) % 16;
            let imm105 = (instruction >> 25) % 64;
            let imm12 = instruction >> 31;
            let value = (imm12 << 12) + (imm11 << 11) + (imm105 << 5) + (imm41 << 1);
            sign_extension(value, 12)
        }
        RiscvForm::U => instruction - (instruction % 2u32.pow(12)),
        RiscvForm::J => {
            let imm1912 = (instruction >> 12) % 256;
            let imm11 = (instruction >> 20) % 2;
            let imm101 = (instruction >> 21) % 1024;
            let imm20 = instruction >> 31;
            let value = (imm20 << 20) + (imm1912 << 12) + (imm11 << 11) + (imm101 << 1);
            sign_extension(value, 20)
        }
        _ => 0,
    }
}

pub fn decode(instruction: u32) -> Result<Instruction, ProcessorError> {
    let funct7 = ((instruction >> 25) % 128) as u8;
    let funct3 = ((instruction >> 12) % 8) as u8;
    let opecode = (instruction % 128) as u8;

    let code = InstructionCode::try_from((funct7, funct3, opecode))?;

    let form = RiscvForm::try_from(opecode)?;

    let rd = ((instruction >> 7) % 32) as u8;
    let rs1 = ((instruction >> 15) % 32) as u8;
    let rs2 = ((instruction >> 20) % 32) as u8;
    let registers = [rs1, rs2, rd];

    let imm = immediate(instruction, &form);

    Ok(Instruction::new(instruction, code, form, registers, imm))
}

#[cfg(test)]
mod tests {
    use crate::processor::decoder::{instruction::RiscvForm, sign_extension};

    use super::immediate;

    #[test]
    fn test_sign_extension() {
        assert_eq!(sign_extension(1, 1), 1);
        assert_eq!(sign_extension(1, 0), u32::MAX);
        assert_eq!(sign_extension(2, 1), u32::MAX - 1);
        assert_eq!(sign_extension(2, 0), 0);
        assert_eq!(sign_extension(8, 0), 0);
        assert_eq!(sign_extension(8, 1), 0);
        assert_eq!(sign_extension(8, 2), 0);
        assert_eq!(sign_extension(8, 3), u32::MAX - 7);
        assert_eq!(sign_extension(0x808, 11), 0xFFFFF808);
        assert_eq!(sign_extension(0x008, 11), 8);
    }

    #[test]
    fn test_immediate() {
        assert_eq!(immediate(0xFEDCBC37, &RiscvForm::U), 0xFEDCB000);
        assert_eq!(immediate(0xFEDCBC6F, &RiscvForm::J), 0xFFFCBFEC);
        assert_eq!(immediate(0xABC00067, &RiscvForm::I), 0xFFFFFABC);
        assert_eq!(immediate(0xDC000A63, &RiscvForm::B), 0xFFFFF5D4);
        assert_eq!(immediate(0x9E000DA3, &RiscvForm::S), 0xFFFFF9FB);
        assert_eq!(immediate(0x01A01013, &RiscvForm::I), 0xFFFFFFFA);
        assert_eq!(immediate(0x40A05013, &RiscvForm::I), 0x0000000A);
    }
}
