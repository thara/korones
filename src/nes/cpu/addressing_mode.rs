use crate::nes::Nes;

use super::{page_crossed, EmuImpl};

// 6502 addressing modes
/// https://wiki.nesdev.org/w/index.php?title=CPU_addressing_modes
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[rustfmt::skip]
#[allow(dead_code)]
pub enum AddressingMode {
    Implicit,
    Accumulator,
    Immediate,
    ZeroPage, ZeroPageX, ZeroPageY,
    Absolute,
    AbsoluteX { penalty: bool },
    AbsoluteY { penalty: bool },
    Relative,
    Indirect, IndexedIndirect, IndirectIndexed { penalty: bool }
}

pub(super) trait GetOperand {
    fn get_operand(&mut self, nes: &mut Nes, mode: AddressingMode) -> u16;
}

impl<T: EmuImpl> GetOperand for T {
    fn get_operand(&mut self, nes: &mut Nes, mode: AddressingMode) -> u16 {
        use AddressingMode::*;

        match mode {
            Implicit => 0,
            Accumulator => nes.cpu.a as u16,
            Immediate => {
                let pc = nes.cpu.pc;
                nes.cpu.incr_pc(1);
                pc
            }
            ZeroPage => {
                let v = self.read(nes, nes.cpu.pc);
                nes.cpu.incr_pc(1);
                v as u16
            }
            ZeroPageX => {
                let pc = self.read(nes, nes.cpu.pc) as u16;
                let x = self.read(nes, nes.cpu.x as u16) as u16;
                nes.cpu.incr_pc(1);
                (pc + x) & 0xFF
            }
            ZeroPageY => {
                let pc = self.read(nes, nes.cpu.pc) as u16;
                let y = self.read(nes, nes.cpu.y as u16) as u16;
                nes.cpu.incr_pc(1);
                (pc + y) & 0xFF
            }
            Absolute => {
                let v = self.read_word(nes, nes.cpu.pc);
                nes.cpu.incr_pc(2);
                v
            }
            AbsoluteX { penalty } => {
                let v = self.read_word(nes, nes.cpu.pc);
                nes.cpu.incr_pc(2);
                if !penalty || page_crossed(nes.cpu.x as u16, v) {
                    self.tick(nes);
                }
                v + nes.cpu.x as u16
            }
            AbsoluteY { penalty } => {
                let v = self.read_word(nes, nes.cpu.pc);
                nes.cpu.incr_pc(2);
                if !penalty && page_crossed(nes.cpu.y as u16, v) {
                    self.tick(nes);
                }
                v + nes.cpu.y as u16
            }
            Relative => {
                let v = self.read(nes, nes.cpu.pc);
                nes.cpu.incr_pc(1);
                v as u16
            }
            Indirect => {
                let m = self.read(nes, nes.cpu.pc);
                let v = self.read_on_indirect(nes, m as u16);
                nes.cpu.incr_pc(2);
                v
            }
            IndexedIndirect => {
                let m = self.read(nes, nes.cpu.pc);
                let v = self.read_on_indirect(nes, (m + nes.cpu.x) as u16);
                nes.cpu.incr_pc(2);
                v
            }
            IndirectIndexed { penalty } => {
                let m = self.read(nes, nes.cpu.pc);
                let v = self.read_on_indirect(nes, m as u16);
                nes.cpu.incr_pc(1);
                if !penalty || page_crossed(nes.cpu.y as u16, v) {
                    self.tick(nes);
                }
                v + nes.cpu.y as u16
            }
        }
    }
}
