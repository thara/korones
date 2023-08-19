use crate::memory_map::MemoryMap;

use super::decode::decode;
use super::{CpuAccess, CpuContext, CpuTickHandler};

struct CpuBus<T: CpuAccess, M: MemoryMap<T>, F: CpuTickHandler<State = T>> {
    context: CpuContext<T, M, F>,
}

impl<T: CpuAccess, M: MemoryMap<T>, F: CpuTickHandler<State = T>> MemoryMap<T> for CpuBus<T, M, F> {
    fn read(cpu: &mut T, addr: u16) -> u8 {
        let v = M::read(cpu, addr);
        F::on_tick(cpu);
        v
    }

    fn write(cpu: &mut T, addr: u16, value: u8) {
        M::write(cpu, addr, value);
        F::on_tick(cpu);
    }
}
