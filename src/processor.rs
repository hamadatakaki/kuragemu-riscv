pub mod decoder;
pub mod executer;
pub mod fetcher;
pub mod register;

use std::path::Path;

use anyhow::Result;

use crate::memory::MainMemory;

use self::{
    decoder::{decode, instruction::InstructionCode},
    executer::execute,
    fetcher::{Fetcher, INSTRUCTION_MEMORY_INIT},
    register::Register,
};

pub struct Processor {
    fetcher: Fetcher,
    register: Register,
    memory: MainMemory,
    is_halt: bool,
}

impl Processor {
    pub fn new() -> Self {
        Self {
            fetcher: Fetcher::new(),
            register: Register::new(),
            memory: MainMemory::new(),
            is_halt: false,
        }
    }

    pub fn init<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let mut fetcher = Fetcher::new();
        fetcher.load_hex(path, INSTRUCTION_MEMORY_INIT)?;

        let cpu = Self {
            fetcher,
            register: Register::new(),
            memory: MainMemory::new(),
            is_halt: false,
        };
        Ok(cpu)
    }

    pub fn step(&mut self) -> Result<(), ProcessorError> {
        // fetch
        let inst = self.fetcher.fetch();
        let pc = self.fetcher.pc;

        println!("[fetch] instruction: {:#06x} | {:#010x}", pc, inst);

        if inst == 0x0000006f {
            // halt:
            self.is_halt = true;
        }

        // decode
        let inst = decode(inst)?;

        println!("[decode] {}", inst);

        if inst.is_halt {
            self.is_halt = true;
            return Ok(());
        }

        // register read
        let rs1 = self.register.read(inst.rs1)?;
        let rs2 = self.register.read(inst.rs2)?;

        println!("[reg read] rs1 = {}(@{:#04x})", rs1, inst.rs1);
        println!("[reg read] rs2 = {}(@{:#04x})", rs2, inst.rs2);

        // execution
        let (mut rd, pc) = execute(&inst, rs1, rs2, pc);

        println!("[exec] (rd, pc) = ({}, {:#04x})", rd, pc);

        // memory read/write
        match &inst.code {
            InstructionCode::Load(opt) => {
                rd = self.memory.read(rs1.wrapping_add(inst.imm), &opt)?;

                println!(
                    "[mem read] rd(@{:#04x}) = mem[{} + {}] = mem[{}({:#010x})] = {}",
                    inst.rd,
                    rs1,
                    inst.imm,
                    rs1.wrapping_add(inst.imm),
                    rs1.wrapping_add(inst.imm),
                    rd
                );
            }
            InstructionCode::Store(opt) => {
                self.memory.write(rs1.wrapping_add(inst.imm), rs2, &opt)?;

                println!(
                    "[mem write] mem[{} + {}] = mem[{}] <= rs2(@{:#04x}) = {}",
                    rs1,
                    inst.imm,
                    rs1.wrapping_add(inst.imm),
                    inst.rs2,
                    rs2
                );
            }
            _ => (),
        }

        // register write
        match &inst.code {
            InstructionCode::Ope(_)
            | InstructionCode::OpeI(_)
            | InstructionCode::Lui
            | InstructionCode::Auipc
            | InstructionCode::Jal
            | InstructionCode::Jalr
            | InstructionCode::Load(_) => {
                self.register.write(inst.rd, rd)?;

                println!("[reg write] rd(@{:#04x}) = {}", inst.rd, rd);
            }
            _ => {}
        }

        // update pc
        self.fetcher.update_program_counter(pc);

        println!("");

        Ok(())
    }

    pub fn is_halt(&self) -> bool {
        self.is_halt
    }

    pub fn logging(&self) {
        self.register.logging().ok().unwrap();
        let mem = self.memory.head(10);
        println!("main: {:?}", mem);
    }
}

pub trait ProcessorErrorTrait {
    fn form(&self) -> String;
}
pub type ProcessorError = Box<dyn ProcessorErrorTrait>;
