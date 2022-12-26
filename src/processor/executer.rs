use self::unit::{alu, branch_operation};

use super::decoder::instruction::{Instruction, InstructionCode};

pub mod unit;

pub fn execute(inst: &Instruction, rs1: u32, rs2: u32, pc: u32) -> (u32, u32) {
    match &inst.code {
        InstructionCode::Ope(code) => (alu(code, rs1, rs2), pc + 4),
        InstructionCode::OpeI(code) => (alu(code, rs1, inst.imm), pc + 4),
        InstructionCode::Lui => (inst.imm, pc + 4),
        InstructionCode::Auipc => (pc.wrapping_add(inst.imm), pc + 4),
        InstructionCode::Branch(option) => {
            let next_pc = if branch_operation(option, rs1, rs2) {
                pc.wrapping_add(inst.imm)
            } else {
                pc + 4
            };

            (0, next_pc)
        }
        InstructionCode::Jal => (pc + 4, pc.wrapping_add(inst.imm)),
        InstructionCode::Jalr => (pc + 4, rs1.wrapping_add(inst.imm)),
        // InstructionCode::Load(option) => {}
        // InstructionCode::Store(option) => {}
        _ => (0, pc + 4),
    }
}
