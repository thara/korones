use crate::cpu::{Cpu, CpuAccess, CpuEmulator, CpuTickHandler};
use crate::memory_map::MemoryMap;

#[derive(Debug, Default)]
pub struct Nes {
    cpu: Cpu,
}

impl CpuAccess for Nes {
    fn get_cpu_mut(&mut self) -> &mut Cpu {
        &mut self.cpu
    }
}

fn step(nes: &mut Nes) {
    CpuEmulator::<Nes, Bus, NesTickHandler>::step(nes);
}

struct Bus;

impl MemoryMap<Nes> for Bus {
    fn read(nes: &mut Nes, addr: u16) -> u8 {
        0
    }

    fn write(nes: &mut Nes, addr: u16, value: u8) {}
}

struct NesTickHandler;

impl CpuTickHandler for NesTickHandler {
    type State = Nes;

    fn on_tick(state: &mut Nes) {}
}
