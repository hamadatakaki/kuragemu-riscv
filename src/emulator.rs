use std::path::Path;

use anyhow::Result;

use super::processor::Processor;

pub struct Emulator {
    cpu: Processor,
}

impl Emulator {
    pub fn new() -> Self {
        let cpu = Processor::new();

        Self { cpu }
    }

    pub fn init<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let cpu = Processor::init(path)?;
        let emu = Self { cpu };

        Ok(emu)
    }

    pub fn run(&mut self) {
        let mut count = 0;

        loop {
            if self.cpu.is_halt() {
                break;
            }

            println!("step: {}", count + 1);
            let res = self.cpu.step();
            match res {
                Err(error) => {
                    println!("{}", error.form());
                    break;
                }
                _ => {}
            }

            count += 1;
        }

        self.cpu.logging();
    }
}
