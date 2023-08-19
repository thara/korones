use crate::memory_map::MemoryMap;

use std::marker::PhantomData;

mod bus;
mod decode;
mod emulator;

pub(crate) use emulator::CpuEmulator;

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

pub(crate) trait CpuAccess {
    fn get_cpu_mut(&mut self) -> &mut Cpu;
}

pub(crate) trait CpuTickHandler {
    type State;

    fn on_tick(state: &mut Self::State);
}

/// `CpuContext` encapsulates the phantom types related to CPU operations.
///
/// Phantom types allow us to define generic behaviors without the need to use them in the runtime.
/// In this context, they are used to represent the constraints and relations between different CPU components
/// like CPU access, memory mapping, and tick handling.
struct CpuContext<T: CpuAccess, M: MemoryMap<T>, F: CpuTickHandler<State = T>> {
    t: PhantomData<T>,
    m: PhantomData<M>,
    f: PhantomData<F>,
}

type Instruction = (Mnemonic, AddressingMode);

// 6502 addressing modes
/// https://wiki.nesdev.org/w/index.php?title=CPU_addressing_modes
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[rustfmt::skip]
enum AddressingMode {
    Implicit,
    Accumulator,
    Immediate,
    ZeroPage, ZeroPageX, ZeroPageY,
    Absolute,
    AbsoluteX { oops: bool },
    AbsoluteY { oops: bool },
    Relative,
    Indirect, IndexedIndirect, IndirectIndexed { oops: bool }
}

// Mnemonics of CPU instructions
#[derive(Debug, Eq, PartialEq)]
#[rustfmt::skip]
#[allow(clippy::upper_case_acronyms)]
enum Mnemonic {
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
