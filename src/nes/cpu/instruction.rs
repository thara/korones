use crate::nes::Nes;

use super::addressing_mode::AddressingMode;
use super::{page_crossed, EmuImpl, Status};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[rustfmt::skip]
#[allow(clippy::upper_case_acronyms)]
pub(crate) enum Mnemonic {
    // Load/Store Operations
    LDA, LDX, LDY, STA, STX, STY,
    // Register Operations
    TAX, TSX, TAY, TXA, TXS, TYA,
    // Stack instructions
    PHA, PHP, PLA, PLP,
    // Logical instructions
    AND, EOR, ORA, BIT,
    // Arithmetic instructions
    ADC, SBC, CMP, CPX, CPY,
    // Increment/Decrement instructions
    INC, INX, INY, DEC, DEX, DEY,
    // Shift instructions
    ASL, LSR, ROL, ROR,
    // Jump instructions
    JMP, JSR, RTS, RTI,
    // Branch instructions
    BCC, BCS, BEQ, BMI, BNE, BPL, BVC, BVS,
    // Flag control instructions
    CLC, CLD, CLI, CLV, SEC, SED, SEI,
    // Misc
    BRK, NOP,
    // Unofficial
    LAX, SAX, DCP, ISB, SLO, RLA, SRE, RRA,
}

pub(super) trait ExecuteInstruction {
    fn execute(&mut self, nes: &mut Nes, inst: (Mnemonic, AddressingMode), operand: u16);
}

impl<T: EmuImpl> ExecuteInstruction for T {
    fn execute(
        &mut self,
        nes: &mut Nes,
        (instruction, mode): (Mnemonic, AddressingMode),
        operand: u16,
    ) {
        use Mnemonic::*;

        match instruction {
            LDA => {
                nes.cpu.a = self.read(nes, operand);
                nes.cpu.p.set_zn(nes.cpu.a);
            }
            LDX => {
                nes.cpu.x = self.read(nes, operand);
                nes.cpu.p.set_zn(nes.cpu.x);
            }
            LDY => {
                nes.cpu.y = self.read(nes, operand);
                nes.cpu.p.set_zn(nes.cpu.y);
            }
            STA => {
                self.write(nes, operand, nes.cpu.a);
            }
            STX => {
                self.write(nes, operand, nes.cpu.x);
            }
            STY => {
                self.write(nes, operand, nes.cpu.y);
            }
            TAX => {
                nes.cpu.x = nes.cpu.a;
                nes.cpu.p.set_zn(nes.cpu.x);
                self.tick(nes);
            }
            TAY => {
                nes.cpu.y = nes.cpu.a;
                nes.cpu.p.set_zn(nes.cpu.y);
                self.tick(nes);
            }
            TXA => {
                nes.cpu.a = nes.cpu.x;
                nes.cpu.p.set_zn(nes.cpu.a);
                self.tick(nes);
            }
            TYA => {
                nes.cpu.a = nes.cpu.y;
                nes.cpu.p.set_zn(nes.cpu.a);
                self.tick(nes);
            }
            TSX => {
                nes.cpu.x = nes.cpu.s;
                nes.cpu.p.set_zn(nes.cpu.x);
                self.tick(nes);
            }
            TXS => {
                nes.cpu.s = nes.cpu.x;
                self.tick(nes);
            }
            PHA => {
                self.push_stack(nes, nes.cpu.a);
                self.tick(nes);
            }
            PHP => {
                let p = nes.cpu.p.bits | Status::OPERATED_B.bits;
                self.push_stack(nes, p);
                self.tick(nes);
            }
            PLA => {
                nes.cpu.a = self.pull_stack(nes);
                nes.cpu.p.set_zn(nes.cpu.a);
                self.tick(nes);
                self.tick(nes);
            }
            PLP => {
                let mut v = self.pull_stack(nes) | !Status::OPERATED_B.bits;
                v |= 0b100000; // for nestest
                nes.cpu.p = Status::from_bits_truncate(v);
                self.tick(nes);
                self.tick(nes);
            }
            AND => and(self, nes, operand),
            EOR => eor(self, nes, operand),
            ORA => ora(self, nes, operand),
            BIT => {
                let m = self.read(nes, operand);
                let b = nes.cpu.a & m;
                nes.cpu.p.set(Status::Z, b == 0);
                nes.cpu.p.set(Status::V, m & 0x40 == 0x40);
                nes.cpu.p.set(Status::N, m & 0x80 == 0x80);
            }
            ADC => adc(self, nes, operand),
            SBC => sbc(self, nes, operand),
            CMP => cmp(self, nes, nes.cpu.a, operand),
            CPX => cmp(self, nes, nes.cpu.x, operand),
            CPY => cmp(self, nes, nes.cpu.y, operand),

            INC => {
                let m = self.read(nes, operand);
                let r = m.wrapping_add(1);
                self.write(nes, operand, r);
                nes.cpu.p.set_zn(r);
                self.tick(nes);
            }
            INX => {
                nes.cpu.x = nes.cpu.x.wrapping_add(1);
                nes.cpu.p.set_zn(nes.cpu.x);
                self.tick(nes);
            }
            INY => {
                nes.cpu.y = nes.cpu.y.wrapping_add(1);
                nes.cpu.p.set_zn(nes.cpu.y);
                self.tick(nes);
            }
            DEC => {
                let m = self.read(nes, operand);
                let r = m.wrapping_add(1);
                self.write(nes, operand, r);
                nes.cpu.p.set_zn(r);
                self.tick(nes);
            }
            DEX => {
                nes.cpu.x = nes.cpu.x.wrapping_sub(1);
                nes.cpu.p.set_zn(nes.cpu.x);
                self.tick(nes);
            }
            DEY => {
                nes.cpu.y = nes.cpu.y.wrapping_sub(1);
                nes.cpu.p.set_zn(nes.cpu.y);
                self.tick(nes);
            }
            ASL => {
                if mode == AddressingMode::Accumulator {
                    nes.cpu.a = asl(nes, nes.cpu.a);
                    self.tick(nes);
                } else {
                    let m = self.read(nes, operand);
                    let r = asl(nes, m);
                    self.tick(nes);
                    self.write(nes, operand, r);
                }
            }
            LSR => {
                if mode == AddressingMode::Accumulator {
                    nes.cpu.a = lsr(nes, nes.cpu.a);
                    self.tick(nes);
                } else {
                    let m = self.read(nes, operand);
                    let r = lsr(nes, m);
                    self.tick(nes);
                    self.write(nes, operand, r);
                }
            }
            ROL => {
                if mode == AddressingMode::Accumulator {
                    nes.cpu.a = rol(nes, nes.cpu.a);
                    self.tick(nes);
                } else {
                    let m = self.read(nes, operand);
                    let r = rol(nes, m);
                    self.tick(nes);
                    self.write(nes, operand, r);
                }
            }
            ROR => {
                if mode == AddressingMode::Accumulator {
                    nes.cpu.a = ror(nes, nes.cpu.a);
                    self.tick(nes);
                } else {
                    let m = self.read(nes, operand);
                    let r = ror(nes, m);
                    self.tick(nes);
                    self.write(nes, operand, r);
                }
            }
            JMP => {
                nes.cpu.pc = operand;
            }
            JSR => {
                let rtn = nes.cpu.pc.wrapping_sub(1);
                self.push_stack_word(nes, rtn);
                nes.cpu.pc = operand;
                self.tick(nes);
            }
            RTS => {
                nes.cpu.pc = self.pull_stack_word(nes);
                nes.cpu.incr_pc(1);
                self.tick(nes);
                self.tick(nes);
                self.tick(nes);
            }

            BCC => branch(self, nes, operand, !nes.cpu.p.contains(Status::C)),
            BCS => branch(self, nes, operand, nes.cpu.p.contains(Status::C)),
            BEQ => branch(self, nes, operand, nes.cpu.p.contains(Status::Z)),
            BMI => branch(self, nes, operand, nes.cpu.p.contains(Status::N)),
            BNE => branch(self, nes, operand, !nes.cpu.p.contains(Status::Z)),
            BPL => branch(self, nes, operand, !nes.cpu.p.contains(Status::N)),
            BVC => branch(self, nes, operand, !nes.cpu.p.contains(Status::V)),
            BVS => branch(self, nes, operand, nes.cpu.p.contains(Status::V)),

            CLC => {
                nes.cpu.p.remove(Status::C);
                self.tick(nes);
            }
            CLD => {
                nes.cpu.p.remove(Status::D);
                self.tick(nes);
            }
            CLI => {
                nes.cpu.p.remove(Status::I);
                self.tick(nes);
            }
            CLV => {
                nes.cpu.p.remove(Status::V);
                self.tick(nes);
            }
            SEC => {
                nes.cpu.p.insert(Status::C);
                self.tick(nes);
            }
            SED => {
                nes.cpu.p.insert(Status::D);
                self.tick(nes);
            }
            SEI => {
                nes.cpu.p.insert(Status::I);
                self.tick(nes);
            }

            BRK => {
                self.push_stack_word(nes, nes.cpu.pc);
                nes.cpu.p.insert(Status::OPERATED_B);
                self.push_stack(nes, nes.cpu.p.bits);
                nes.cpu.pc = self.read_word(nes, 0xFFFE);
                self.tick(nes);
            }
            NOP => {
                self.tick(nes);
            }
            RTI => {
                let v = self.pull_stack(nes);
                nes.cpu.p = Status::from_bits_truncate(v);
                nes.cpu.pc = self.pull_stack_word(nes);
                self.tick(nes);
                self.tick(nes);
            }

            LAX => {
                let m = self.read(nes, operand);
                nes.cpu.a = m;
                nes.cpu.p.set_zn(m);
                nes.cpu.x = m;
            }
            SAX => self.write(nes, operand, nes.cpu.a & nes.cpu.x),
            DCP => {
                // decrementMemory excluding tick
                let m = self.read(nes, operand).wrapping_sub(1);
                nes.cpu.p.set_zn(m);
                self.write(nes, operand, m);
                cmp(self, nes, nes.cpu.a, operand);
            }
            ISB => {
                // incrementMemory excluding tick
                let m = self.read(nes, operand).wrapping_add(1);
                nes.cpu.p.set_zn(m);
                self.write(nes, operand, m);
                sbc(self, nes, operand);
            }
            SLO => {
                // arithmeticShiftLeft excluding tick
                let m = self.read(nes, operand);
                nes.cpu.p.set(Status::C, m & 0x80 == 0x80);
                self.write(nes, operand, m << 1);
                ora(self, nes, operand);
            }
            RLA => {
                // rotateLeft excluding tick
                let m = self.read(nes, operand);
                let c = m & 0x80;
                let mut r = m << 1;
                if nes.cpu.p.contains(Status::C) {
                    r |= 1;
                }
                nes.cpu.p.set(Status::C, c == 0x80);
                nes.cpu.p.set_zn(r);
                self.write(nes, operand, r);
                and(self, nes, operand);
            }
            SRE => {
                // logicalShiftRight excluding tick
                let m = self.read(nes, operand);
                nes.cpu.p.set(Status::C, m & 1 == 1);
                let r = m >> 1;
                nes.cpu.p.set_zn(r);
                self.write(nes, operand, r);
                eor(self, nes, operand);
            }
            RRA => {
                // rotateRight excluding tick
                let m = self.read(nes, operand);
                let c = m & 1;
                let mut r = m >> 1;
                if nes.cpu.p.contains(Status::C) {
                    r |= 0x80;
                }
                nes.cpu.p.set(Status::C, c == 1);
                nes.cpu.p.set_zn(r);
                self.write(nes, operand, r);
                adc(self, nes, operand);
            }
        }
    }
}

fn and<E: EmuImpl>(e: &mut E, nes: &mut Nes, v: u16) {
    nes.cpu.a &= e.read(nes, v);
    nes.cpu.p.set_zn(nes.cpu.a);
}

fn eor<E: EmuImpl>(e: &mut E, nes: &mut Nes, v: u16) {
    nes.cpu.a ^= e.read(nes, v);
    nes.cpu.p.set_zn(nes.cpu.a);
}

fn ora<E: EmuImpl>(e: &mut E, nes: &mut Nes, v: u16) {
    nes.cpu.a |= e.read(nes, v);
    nes.cpu.p.set_zn(nes.cpu.a);
}

fn carry(nes: &mut Nes, m: u8, r: u8) {
    let a7 = nes.cpu.a >> 7 & 1;
    let m7 = m >> 7 & 1;
    let c6 = a7 ^ m7 ^ (r >> 7 & 1);
    let c7 = (a7 & m7) | (a7 & c6) | (m7 & c6);
    nes.cpu.p.set(Status::C, c7 == 1);
    nes.cpu.p.set(Status::V, c6 ^ c7 == 1);
}

fn adc<E: EmuImpl>(e: &mut E, nes: &mut Nes, v: u16) {
    let m = e.read(nes, v);
    let mut r = nes.cpu.a + m;
    if nes.cpu.p.contains(Status::C) {
        r = r.wrapping_add(1);
    }
    carry(nes, m, r);
    nes.cpu.a = r;
    nes.cpu.p.set_zn(nes.cpu.a);
}

fn sbc<E: EmuImpl>(e: &mut E, nes: &mut Nes, v: u16) {
    let m = !e.read(nes, v);
    let mut r = nes.cpu.a + m;
    if nes.cpu.p.contains(Status::C) {
        r = r.wrapping_add(1);
    }
    carry(nes, m, r);
    nes.cpu.a = r;
    nes.cpu.p.set_zn(r);
}

fn cmp<E: EmuImpl>(e: &mut E, nes: &mut Nes, x: u8, v: u16) {
    let r = x as i16 - e.read(nes, v) as i16;
    nes.cpu.p.set_zn(r as u8);
    nes.cpu.p.set(Status::C, 0 <= r);
}

fn asl(nes: &mut Nes, m: u8) -> u8 {
    nes.cpu.p.set(Status::C, m & 0x80 == 0x80);
    let r = m << 1;
    nes.cpu.p.set_zn(r);
    r
}

fn lsr(nes: &mut Nes, m: u8) -> u8 {
    nes.cpu.p.set(Status::C, m & 1 == 1);
    let r = m >> 1;
    nes.cpu.p.set_zn(r);
    r
}

fn rol(nes: &mut Nes, m: u8) -> u8 {
    let c = m & 0x80;
    let mut r = m << 1;
    if nes.cpu.p.contains(Status::C) {
        r |= 1;
    }
    nes.cpu.p.set(Status::C, c == 0x80);
    nes.cpu.p.set_zn(r);
    r
}

fn ror(nes: &mut Nes, m: u8) -> u8 {
    let c = m & 1;
    let mut r = m >> 1;
    if nes.cpu.p.contains(Status::C) {
        r |= 0x80;
    }
    nes.cpu.p.set(Status::C, c == 1);
    nes.cpu.p.set_zn(r);
    r
}

fn branch<E: EmuImpl>(e: &mut E, nes: &mut Nes, v: u16, cond: bool) {
    if !cond {
        return;
    }
    e.tick(nes);
    let base = nes.cpu.pc as i16;
    let offset = v as i8; // to negative number
    if page_crossed(offset as u16, base as u16) {
        e.tick(nes);
    }
    nes.cpu.pc = (base + offset as i16) as u16;
}
