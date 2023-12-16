use crate::cpu::emu::{CpuEmu, CpuTickHandler};
use crate::memory_map::MemoryMap;

use super::Nes;

pub fn step(nes: &mut Nes) {
    CpuEmu::<Bus, TickHandler>::step(nes);
}

struct Bus;

impl MemoryMap for Bus {
    fn read(nes: &mut Nes, addr: u16) -> u8 {
        0
    }

    fn write(nes: &mut Nes, addr: u16, value: u8) {}
}

struct TickHandler;

impl CpuTickHandler for TickHandler {
    fn on_tick(state: &mut Nes) {}
}
