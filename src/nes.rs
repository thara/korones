mod cpu;
use crate::nes::cpu::Emu as cpuEmu;

struct Nes {
    cpu: cpu::Cpu,

    cpu_cycle: u128,
    cpu_wram: [u8; 0x2000],

    // current interrupt status
    //
    // `None` represents to no interrupted.
    interrupt: Option<Interrupt>,
}

/// Kinds of CPU interrupts
///
/// It currently supports NMI and IRQ only.
#[derive(Debug, Clone, Copy)]
#[allow(dead_code, clippy::upper_case_acronyms)]
pub enum Interrupt {
    NMI,
    IRQ,
}

struct Emu {}

impl Emu {
    fn step(&mut self, nes: &mut Nes) {
        self.cpu_step(nes);
    }
}

impl cpu::EmuImpl for Emu {}

impl cpu::MemoryMap for Emu {
    fn cpu_read(&mut self, nes: &mut Nes, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1FFF => nes.cpu_wram[addr as usize],
            //TODO
            _ => 0u8,
        }
    }

    fn cpu_write(&mut self, nes: &mut Nes, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1FFF => nes.cpu_wram[addr as usize] = value,
            //TODO
            _ => {}
        }
    }
}

impl cpu::TickHandler for Emu {
    fn on_cpu_tick(&mut self, nes: &mut Nes) {
        nes.cpu_cycle = nes.cpu_cycle.wrapping_add(1);
    }
}
