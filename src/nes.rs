use crate::cpu::Cpu;
use crate::memory_map::MemoryMap;

mod emu;

#[derive(Debug, Default)]
pub struct Nes {
    pub(crate) cpu: Cpu,
}
