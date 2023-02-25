use crate::cpu::Cpu;

pub struct Nes {
    pub(crate) cpu: Cpu,
    // current interrupt status
    //
    // `None` represents to no interrupted.
    pub(crate) interrupt: Option<Interrupt>,
}

/// Kinds of CPU interrupts
///
/// It currently supports NMI and IRQ only.
#[derive(Debug, Clone, Copy)]
#[allow(dead_code, clippy::upper_case_acronyms)]
pub(crate) enum Interrupt {
    NMI,
    IRQ,
}

impl Nes {
    pub fn step(&mut self) {
        use crate::cpu::CpuEmu;
        use crate::emu::Emu;

        Emu::cpu_step(self);
    }
}
