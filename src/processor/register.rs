use std::fmt::Display;

use anyhow::Result;

use super::{decoder::instruction::RiscvInstruction, ProcessorError, ProcessorErrorTrait};

pub enum RegisterAlias {
    Zero,
    RA,
    SP,
    GP,
    TP,
    T0,
    T1,
    T2,
    S0,
    S1,
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    S10,
    S11,
    T3,
    T4,
    T5,
    T6,
}

impl RiscvInstruction for RegisterAlias {
    fn assembly(&self) -> String {
        match self {
            RegisterAlias::Zero => "zero",
            RegisterAlias::RA => "ra",
            RegisterAlias::SP => "sp",
            RegisterAlias::GP => "gp",
            RegisterAlias::TP => "tp",
            RegisterAlias::T0 => "t0",
            RegisterAlias::T1 => "t1",
            RegisterAlias::T2 => "t2",
            RegisterAlias::S0 => "s0",
            RegisterAlias::S1 => "s1",
            RegisterAlias::A0 => "a0",
            RegisterAlias::A1 => "a1",
            RegisterAlias::A2 => "a2",
            RegisterAlias::A3 => "a3",
            RegisterAlias::A4 => "a4",
            RegisterAlias::A5 => "a5",
            RegisterAlias::A6 => "a6",
            RegisterAlias::A7 => "a7",
            RegisterAlias::S2 => "s2",
            RegisterAlias::S3 => "s3",
            RegisterAlias::S4 => "s4",
            RegisterAlias::S5 => "s5",
            RegisterAlias::S6 => "s6",
            RegisterAlias::S7 => "s7",
            RegisterAlias::S8 => "s8",
            RegisterAlias::S9 => "s9",
            RegisterAlias::S10 => "s10",
            RegisterAlias::S11 => "s11",
            RegisterAlias::T3 => "t3",
            RegisterAlias::T4 => "t4",
            RegisterAlias::T5 => "t5",
            RegisterAlias::T6 => "t6",
        }
        .into()
    }
}

impl TryFrom<u8> for RegisterAlias {
    type Error = ProcessorError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value < 32 {
            let alias = match value {
                0 => RegisterAlias::Zero,
                1 => RegisterAlias::RA,
                2 => RegisterAlias::SP,
                3 => RegisterAlias::GP,
                4 => RegisterAlias::TP,
                5 => RegisterAlias::T0,
                6 => RegisterAlias::T1,
                7 => RegisterAlias::T2,
                8 => RegisterAlias::S0,
                9 => RegisterAlias::S1,
                10 => RegisterAlias::A0,
                11 => RegisterAlias::A1,
                12 => RegisterAlias::A2,
                13 => RegisterAlias::A3,
                14 => RegisterAlias::A4,
                15 => RegisterAlias::A5,
                16 => RegisterAlias::A6,
                17 => RegisterAlias::A7,
                18 => RegisterAlias::S2,
                19 => RegisterAlias::S3,
                20 => RegisterAlias::S4,
                21 => RegisterAlias::S5,
                22 => RegisterAlias::S6,
                23 => RegisterAlias::S7,
                24 => RegisterAlias::S8,
                25 => RegisterAlias::S9,
                26 => RegisterAlias::S10,
                27 => RegisterAlias::S11,
                28 => RegisterAlias::T3,
                29 => RegisterAlias::T4,
                30 => RegisterAlias::T5,
                31 => RegisterAlias::T6,
                _ => unreachable!(),
            };
            Ok(alias)
        } else {
            let error_type = RegisterErrorType::AddressOutOfBounds;
            Err(RegisterError::new(error_type))
        }
    }
}

impl Display for RegisterAlias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.assembly())
    }
}

pub struct Register {
    mem: [u32; 32],
}

impl Register {
    pub fn new() -> Self {
        Self { mem: [0; 32] }
    }

    pub fn read(&self, address: u8) -> Result<u32, ProcessorError> {
        if address < 32 {
            Ok(self.mem[address as usize])
        } else {
            let error_type = RegisterErrorType::AddressOutOfBounds;
            Err(RegisterError::new(error_type))
        }
    }

    pub fn write(&mut self, address: u8, value: u32) -> Result<(), ProcessorError> {
        if address < 32 {
            if 0 < address {
                self.mem[address as usize] = value;
            }
            Ok(())
        } else {
            let error_type = RegisterErrorType::AddressOutOfBounds;
            Err(RegisterError::new(error_type))
        }
    }

    pub fn logging(&self) -> Result<(), ProcessorError> {
        for i in 0..32 {
            let reg = RegisterAlias::try_from(i)?.assembly();
            println!(" {:8}: {}", reg, self.read(i)?);
        }
        Ok(())
    }
}

pub enum RegisterErrorType {
    AddressOutOfBounds,
}

impl Display for RegisterErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AddressOutOfBounds => {
                write!(f, "violate address bounds by memory access")
            }
        }
    }
}

pub struct RegisterError {
    error_type: RegisterErrorType,
}

impl RegisterError {
    fn new(error_type: RegisterErrorType) -> Box<Self> {
        let error = Self { error_type };
        Box::new(error)
    }
}

impl Display for RegisterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.form())
    }
}

impl ProcessorErrorTrait for RegisterError {
    fn form(&self) -> String {
        format!("register access is failed  - {}", self.error_type)
    }
}
