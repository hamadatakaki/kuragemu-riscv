use crate::processor::decoder::instruction::{AluCode, BranchOption};

pub fn alu(code: &AluCode, lhs: u32, rhs: u32) -> u32 {
    match code {
        AluCode::Add => lhs.wrapping_add(rhs),
        AluCode::Sub => lhs.wrapping_sub(rhs),
        AluCode::Slt => ((lhs as i32) < (rhs as i32)) as u32,
        AluCode::Sltu => (lhs < rhs) as u32,
        AluCode::Sll => lhs << rhs,
        AluCode::Srl => lhs >> rhs,
        AluCode::Sra => ((lhs as i32) >> (rhs as i32)) as u32,
        AluCode::Xor => lhs ^ rhs,
        AluCode::Or => lhs | rhs,
        AluCode::And => lhs & rhs,
    }
}

pub fn branch_operation(option: &BranchOption, rs1: u32, rs2: u32) -> bool {
    match option {
        BranchOption::Equal => rs1 == rs2,
        BranchOption::NotEqual => rs1 != rs2,
        BranchOption::GraterEqual => (rs1 as i32) >= (rs2 as i32),
        BranchOption::GraterEqualUnsigned => rs1 >= rs2,
        BranchOption::LessThan => (rs1 as i32) < (rs2 as i32),
        BranchOption::LessThanUnsigned => rs1 < rs2,
    }
}

#[cfg(test)]
mod tests {
    use crate::processor::decoder::instruction::AluCode;

    use super::alu;

    #[test]
    fn test_alu() {
        assert_eq!(alu(&AluCode::Add, 0x1, 0xF), 0x10);
        assert_eq!(alu(&AluCode::Add, 0xFFFFFFFF, 0x2), 0x1);
        assert_eq!(alu(&AluCode::Sub, 0xA, 0x8), 0x2);
        assert_eq!(alu(&AluCode::Slt, 0x1, 0x2), 0x1);
        assert_eq!(alu(&AluCode::Slt, 0xFFFFFFFE, 0), 1);
        assert_eq!(alu(&AluCode::Sltu, 0x1, 0x2), 0x1);
        assert_eq!(alu(&AluCode::Sltu, 0xFFFFFFFE, 0), 0);
        assert_eq!(alu(&AluCode::Sll, 0x1, 0x3), 0x8);
        assert_eq!(alu(&AluCode::Srl, 0x8, 0x2), 0x2);
        assert_eq!(alu(&AluCode::Srl, 0x80000000, 0x3), 0x10000000);
        assert_eq!(alu(&AluCode::Sra, 0x80000000, 0x3), 0xF0000000);
        assert_eq!(alu(&AluCode::Xor, 0b00001111, 0b10101010), 0b10100101);
        assert_eq!(alu(&AluCode::Or, 0b11110000, 0b01010101), 0b11110101);
        assert_eq!(alu(&AluCode::And, 0b00001111, 0b01010101), 0b00000101);
    }
}
