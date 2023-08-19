use crate::memory_map::MemoryMap;

use super::decode::decode;
use super::{CpuAccess, CpuContext, CpuTickHandler, Status};

pub(crate) struct CpuEmulator<T: CpuAccess, M: MemoryMap<T>, F: CpuTickHandler<State = T>> {
    context: CpuContext<T, M, F>,
}

impl<T: CpuAccess, M: MemoryMap<T>, F: CpuTickHandler<State = T>> CpuEmulator<T, M, F> {
    pub fn step(cpu: &mut T) {
        let opcode = Self::fetch(cpu);
        let _ = decode(opcode);
    }

    fn fetch(cpu: &mut T) -> u8 {
        let pc = cpu.get_cpu_mut().pc;
        let op = M::read(cpu, pc);
        cpu.get_cpu_mut().pc += 1;
        op
    }
}

impl Status {
    #[allow(dead_code)]
    fn set_zn(&mut self, v: u8) {
        self.set(Self::Z, v == 0);
        self.set(Self::N, (v >> 7) & 1 == 1);
    }
}
