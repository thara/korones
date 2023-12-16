use crate::memory_map::MemoryMap;

mod decode;
pub mod emu;

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) struct Cpu {
    // https://wiki.nesdev.org/w/index.php?title=CPU_registers

    // Accumulator, Index X/Y register
    a: u8,
    x: u8,
    y: u8,
    // Stack pointer
    s: u8,

    // Status register
    p: Status,

    // Program counter
    pc: u16,

    // CPU cycles
    cycles: u128,
    // CPU internal RAM
    ram: [u8; 0x2000],
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu {
            a: Default::default(),
            x: Default::default(),
            y: Default::default(),
            s: Default::default(),
            p: Default::default(),
            pc: Default::default(),
            cycles: Default::default(),
            ram: [0; 0x2000],
        }
    }
}

bitflags! {
    #[derive(Default)]
    struct Status: u8 {
        // Negative
        const N = 1 << 7;
        // Overflow
        const V = 1 << 6;
        const R = 1 << 5;
        const B = 1 << 4;
        // Decimal mode
        const D = 1 << 3;
        // IRQ prevention
        const I = 1 << 2;
        // Zero
        const Z = 1 << 1;
        // Carry
        const C = 1 << 0;
        // B flags
        // https://wiki.nesdev.com/w/index.php/Status_flags#The_B_flag
        const INTERRUPTED_B = 0b100000; // interruptB
        const OPERATED_B = 0b110000; // instructionB
    }
}
