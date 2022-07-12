use crate::nes::{Interrupt, Nes};

use super::{EmuImpl, Status};

pub(super) trait InterruptHandler {
    fn handle_interrupt(&mut self, nes: &mut Nes);
}

impl<T: EmuImpl> InterruptHandler for T {
    fn handle_interrupt(&mut self, nes: &mut Nes) {
        match nes.interrupt {
            Some(Interrupt::NMI) => on_interrupt(self, nes, 0xFFFA),
            Some(Interrupt::IRQ) if nes.cpu.p.contains(Status::I) => {
                on_interrupt(self, nes, 0xFFFE)
            }
            _ => {}
        }
    }
}

fn on_interrupt<E: EmuImpl>(e: &mut E, nes: &mut Nes, vector: u16) {
    e.tick(nes);
    e.push_stack_word(nes, nes.cpu.pc);
    // https://wiki.nesdev.com/w/index.php/Status_flags#The_B_flag
    // http://visual6502.org/wiki/index.php?title=6502_BRK_and_B_bit
    e.push_stack(nes, nes.cpu.p.bits | Status::INTERRUPTED_B.bits);
    nes.cpu.p.insert(Status::I);
    nes.cpu.pc = e.read_word(nes, vector);
    nes.interrupt = None;
}
