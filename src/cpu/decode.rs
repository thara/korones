use super::{AddressingMode, Instruction, Mnemonic};

pub(super) fn decode(opcode: u8) -> Instruction {
    use AddressingMode::*;
    use Mnemonic::*;

    match opcode {
        0x69 => (ADC, Immediate),
        0x65 => (ADC, ZeroPage),
        0x75 => (ADC, ZeroPageX),
        0x6D => (ADC, Absolute),
        0x7D => (ADC, AbsoluteX { oops: true }),
        0x79 => (ADC, AbsoluteY { oops: true }),
        0x61 => (ADC, IndexedIndirect),
        0x71 => (ADC, IndirectIndexed { oops: true }),

        0x29 => (AND, Immediate),
        0x25 => (AND, ZeroPage),
        0x35 => (AND, ZeroPageX),
        0x2D => (AND, Absolute),
        0x3D => (AND, AbsoluteX { oops: true }),
        0x39 => (AND, AbsoluteY { oops: true }),
        0x21 => (AND, IndexedIndirect),
        0x31 => (AND, IndirectIndexed { oops: true }),

        0x0A => (ASL, Accumulator),
        0x06 => (ASL, ZeroPage),
        0x16 => (ASL, ZeroPageX),
        0x0E => (ASL, Absolute),
        0x1E => (ASL, AbsoluteX { oops: false }),

        0x90 => (BCC, Relative),
        0xB0 => (BCS, Relative),
        0xF0 => (BEQ, Relative),

        0x24 => (BIT, ZeroPage),
        0x2C => (BIT, Absolute),

        0x30 => (BMI, Relative),
        0xD0 => (BNE, Relative),
        0x10 => (BPL, Relative),

        0x00 => (BRK, Implicit),

        0x50 => (BVC, Relative),
        0x70 => (BVS, Relative),

        0x18 => (CLC, Implicit),
        0xD8 => (CLD, Implicit),
        0x58 => (CLI, Implicit),
        0xB8 => (CLV, Implicit),

        0xC9 => (CMP, Immediate),
        0xC5 => (CMP, ZeroPage),
        0xD5 => (CMP, ZeroPageX),
        0xCD => (CMP, Absolute),
        0xDD => (CMP, AbsoluteX { oops: true }),
        0xD9 => (CMP, AbsoluteY { oops: true }),
        0xC1 => (CMP, IndexedIndirect),
        0xD1 => (CMP, IndirectIndexed { oops: true }),

        0xE0 => (CPX, Immediate),
        0xE4 => (CPX, ZeroPage),
        0xEC => (CPX, Absolute),
        0xC0 => (CPY, Immediate),
        0xC4 => (CPY, ZeroPage),
        0xCC => (CPY, Absolute),

        0xC6 => (DEC, ZeroPage),
        0xD6 => (DEC, ZeroPageX),
        0xCE => (DEC, Absolute),
        0xDE => (DEC, AbsoluteX { oops: false }),

        0xCA => (DEX, Implicit),
        0x88 => (DEY, Implicit),

        0x49 => (EOR, Immediate),
        0x45 => (EOR, ZeroPage),
        0x55 => (EOR, ZeroPageX),
        0x4D => (EOR, Absolute),
        0x5D => (EOR, AbsoluteX { oops: true }),
        0x59 => (EOR, AbsoluteY { oops: true }),
        0x41 => (EOR, IndexedIndirect),
        0x51 => (EOR, IndirectIndexed { oops: true }),

        0xE6 => (INC, ZeroPage),
        0xF6 => (INC, ZeroPageX),
        0xEE => (INC, Absolute),
        0xFE => (INC, AbsoluteX { oops: false }),

        0xE8 => (INX, Implicit),
        0xC8 => (INY, Implicit),

        0x4C => (JMP, Absolute),
        0x6C => (JMP, Indirect),

        0x20 => (JSR, Absolute),

        0xA9 => (LDA, Immediate),
        0xA5 => (LDA, ZeroPage),
        0xB5 => (LDA, ZeroPageX),
        0xAD => (LDA, Absolute),
        0xBD => (LDA, AbsoluteX { oops: true }),
        0xB9 => (LDA, AbsoluteY { oops: true }),
        0xA1 => (LDA, IndexedIndirect),
        0xB1 => (LDA, IndirectIndexed { oops: true }),

        0xA2 => (LDX, Immediate),
        0xA6 => (LDX, ZeroPage),
        0xB6 => (LDX, ZeroPageY),
        0xAE => (LDX, Absolute),
        0xBE => (LDX, AbsoluteY { oops: true }),

        0xA0 => (LDY, Immediate),
        0xA4 => (LDY, ZeroPage),
        0xB4 => (LDY, ZeroPageX),
        0xAC => (LDY, Absolute),
        0xBC => (LDY, AbsoluteX { oops: true }),

        0x4A => (LSR, Accumulator),
        0x46 => (LSR, ZeroPage),
        0x56 => (LSR, ZeroPageX),
        0x4E => (LSR, Absolute),
        0x5E => (LSR, AbsoluteX { oops: false }),

        0x09 => (ORA, Immediate),
        0x05 => (ORA, ZeroPage),
        0x15 => (ORA, ZeroPageX),
        0x0D => (ORA, Absolute),
        0x1D => (ORA, AbsoluteX { oops: true }),
        0x19 => (ORA, AbsoluteY { oops: true }),
        0x01 => (ORA, IndexedIndirect),
        0x11 => (ORA, IndirectIndexed { oops: true }),

        0x48 => (PHA, Implicit),
        0x08 => (PHP, Implicit),
        0x68 => (PLA, Implicit),
        0x28 => (PLP, Implicit),

        0x2A => (ROL, Accumulator),
        0x26 => (ROL, ZeroPage),
        0x36 => (ROL, ZeroPageX),
        0x2E => (ROL, Absolute),
        0x3E => (ROL, AbsoluteX { oops: false }),

        0x6A => (ROR, Accumulator),
        0x66 => (ROR, ZeroPage),
        0x76 => (ROR, ZeroPageX),
        0x6E => (ROR, Absolute),
        0x7E => (ROR, AbsoluteX { oops: false }),

        0x40 => (RTI, Implicit),
        0x60 => (RTS, Implicit),

        0xE9 => (SBC, Immediate),
        0xE5 => (SBC, ZeroPage),
        0xF5 => (SBC, ZeroPageX),
        0xED => (SBC, Absolute),
        0xFD => (SBC, AbsoluteX { oops: true }),
        0xF9 => (SBC, AbsoluteY { oops: true }),
        0xE1 => (SBC, IndexedIndirect),
        0xF1 => (SBC, IndirectIndexed { oops: true }),

        0x38 => (SEC, Implicit),
        0xF8 => (SED, Implicit),
        0x78 => (SEI, Implicit),

        0x85 => (STA, ZeroPage),
        0x95 => (STA, ZeroPageX),
        0x8D => (STA, Absolute),
        0x9D => (STA, AbsoluteX { oops: false }),
        0x99 => (STA, AbsoluteY { oops: false }),
        0x81 => (STA, IndexedIndirect),
        0x91 => (STA, IndirectIndexed { oops: false }),

        0x86 => (STX, ZeroPage),
        0x96 => (STX, ZeroPageY),
        0x8E => (STX, Absolute),
        0x84 => (STY, ZeroPage),
        0x94 => (STY, ZeroPageX),
        0x8C => (STY, Absolute),

        0xAA => (TAX, Implicit),
        0xA8 => (TAY, Implicit),
        0xBA => (TSX, Implicit),
        0x8A => (TXA, Implicit),
        0x9A => (TXS, Implicit),
        0x98 => (TYA, Implicit),

        0x04 | 0x44 | 0x64 => (NOP, ZeroPage),
        0x0C => (NOP, Absolute),
        0x14 | 0x34 | 0x54 | 0x74 | 0xD4 | 0xF4 => (NOP, ZeroPageX),
        0x1A | 0x3A | 0x5A | 0x7A | 0xDA | 0xEA | 0xFA => (NOP, Implicit),
        0x1C | 0x3C | 0x5C | 0x7C | 0xDC | 0xFC => (NOP, AbsoluteX { oops: true }),
        0x80 | 0x82 | 0x89 | 0xC2 | 0xE2 => (NOP, Immediate),

        // unofficial
        0xEB => (SBC, Immediate),

        0xA3 => (LAX, IndexedIndirect),
        0xA7 => (LAX, ZeroPage),
        0xAB => (LAX, Immediate),
        0xAF => (LAX, Absolute),
        0xB3 => (LAX, IndirectIndexed { oops: true }),
        0xB7 => (LAX, ZeroPageY),
        0xBF => (LAX, AbsoluteY { oops: true }),

        0x83 => (SAX, IndexedIndirect),
        0x87 => (SAX, ZeroPage),
        0x8F => (SAX, Absolute),
        0x97 => (SAX, ZeroPageY),

        0xC3 => (DCP, IndexedIndirect),
        0xC7 => (DCP, ZeroPage),
        0xCF => (DCP, Absolute),
        0xD3 => (DCP, IndirectIndexed { oops: false }),
        0xD7 => (DCP, ZeroPageX),
        0xDB => (DCP, AbsoluteY { oops: false }),
        0xDF => (DCP, AbsoluteX { oops: false }),

        0xE3 => (ISB, IndexedIndirect),
        0xE7 => (ISB, ZeroPage),
        0xEF => (ISB, Absolute),
        0xF3 => (ISB, IndirectIndexed { oops: false }),
        0xF7 => (ISB, ZeroPageX),
        0xFB => (ISB, AbsoluteY { oops: false }),
        0xFF => (ISB, AbsoluteX { oops: false }),

        0x03 => (SLO, IndexedIndirect),
        0x07 => (SLO, ZeroPage),
        0x0F => (SLO, Absolute),
        0x13 => (SLO, IndirectIndexed { oops: false }),
        0x17 => (SLO, ZeroPageX),
        0x1B => (SLO, AbsoluteY { oops: false }),
        0x1F => (SLO, AbsoluteX { oops: false }),

        0x23 => (RLA, IndexedIndirect),
        0x27 => (RLA, ZeroPage),
        0x2F => (RLA, Absolute),
        0x33 => (RLA, IndirectIndexed { oops: false }),
        0x37 => (RLA, ZeroPageX),
        0x3B => (RLA, AbsoluteY { oops: false }),
        0x3F => (RLA, AbsoluteX { oops: false }),

        0x43 => (SRE, IndexedIndirect),
        0x47 => (SRE, ZeroPage),
        0x4F => (SRE, Absolute),
        0x53 => (SRE, IndirectIndexed { oops: false }),
        0x57 => (SRE, ZeroPageX),
        0x5B => (SRE, AbsoluteY { oops: false }),
        0x5F => (SRE, AbsoluteX { oops: false }),

        0x63 => (RRA, IndexedIndirect),
        0x67 => (RRA, ZeroPage),
        0x6F => (RRA, Absolute),
        0x73 => (RRA, IndirectIndexed { oops: false }),
        0x77 => (RRA, ZeroPageX),
        0x7B => (RRA, AbsoluteY { oops: false }),
        0x7F => (RRA, AbsoluteX { oops: false }),

        _ => (NOP, Implicit),
    }
}
