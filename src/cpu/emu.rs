use std::marker::PhantomData;

use crate::memory_map::MemoryMap;
use crate::nes::Nes;

use super::decode::{decode, Instruction};

pub(crate) trait CpuTickHandler {
    fn on_tick(nes: &mut Nes);
}

pub(crate) struct CpuEmu<M: MemoryMap, T: CpuTickHandler> {
    m: PhantomData<M>,
    t: PhantomData<T>,
}

impl<M: MemoryMap, T: CpuTickHandler> CpuEmu<M, T> {
    pub(crate) fn step(nes: &mut Nes) {
        let op = Self::read(nes, nes.cpu.pc);
        nes.cpu.pc += 1;

        let inst = decode(op);
        Self::execute(nes, inst);
    }

    fn execute(nes: &mut Nes, inst: Instruction) {}
}

impl<M: MemoryMap, T: CpuTickHandler> MemoryMap for CpuEmu<M, T> {
    fn read(nes: &mut Nes, addr: u16) -> u8 {
        let v = M::read(nes, addr);
        T::on_tick(nes);
        v
    }

    fn write(nes: &mut Nes, addr: u16, value: u8) {
        M::write(nes, addr, value);
        T::on_tick(nes);
    }
}
