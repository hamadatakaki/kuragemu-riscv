use anyhow::Result;
use kuragemu::emulator::Emulator;

fn main() -> Result<()> {
    let path = "./example/instructions/ope.hex";
    let mut emu = Emulator::init(path)?;
    emu.run();
    Ok(())
}
