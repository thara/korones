mod addressing_mode;
mod decode;
mod instruction;

use crate::nes::Nes;

/// CPU state
#[derive(Debug, Default, Clone)]
pub(super) struct Cpu {
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
}

impl Cpu {
    fn incr_pc(&mut self, n: u16) {
        self.pc = self.pc.wrapping_add(n);
    }
}

bitflags! {
    #[derive(Default)]
    pub(crate) struct Status: u8 {
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

impl Status {
    fn set_zn(&mut self, v: u8) {
        self.set(Self::Z, v == 0);
        self.set(Self::N, (v >> 7) & 1 == 1);
    }
}

pub(super) trait Emu {
    fn cpu_power_on(&mut self, nes: &mut Nes);
    fn cpu_step(&mut self, nes: &mut Nes);
}

pub(super) trait MemoryMap {
    fn cpu_read(&mut self, nes: &mut Nes, addr: u16) -> u8;
    fn cpu_write(&mut self, nes: &mut Nes, addr: u16, value: u8);
}

pub(super) trait TickHandler {
    fn on_cpu_tick(&mut self, nes: &mut Nes);
}

pub(super) trait EmuImpl: MemoryMap + TickHandler {
    fn fetch(&mut self, nes: &mut Nes) -> u8 {
        let op = self.read(nes, nes.cpu.pc);
        nes.cpu.pc = nes.cpu.pc.wrapping_add(1);
        op
    }

    fn read(&mut self, nes: &mut Nes, addr: u16) -> u8 {
        let v = self.cpu_read(nes, addr);
        self.tick(nes);
        v
    }

    fn read_word(&mut self, nes: &mut Nes, addr: u16) -> u16 {
        let low = self.cpu_read(nes, addr) as u16;
        let high = self.cpu_read(nes, addr + 1) as u16;
        low | (high << 8)
    }

    fn read_on_indirect(&mut self, nes: &mut Nes, addr: u16) -> u16 {
        let low = self.read(nes, addr) as u16;
        // Reproduce 6502 bug - http://nesdev.com/6502bugs.txt
        let high = self.read(nes, (addr & 0xFF00) | ((addr + 1) & 0x00FF)) as u16;
        low | (high << 8)
    }

    fn write(&mut self, nes: &mut Nes, addr: u16, value: u8) {
        self.tick(nes);
        self.cpu_write(nes, addr, value);
    }

    fn tick(&mut self, nes: &mut Nes) {
        self.on_cpu_tick(nes);
    }

    fn push_stack(&mut self, nes: &mut Nes, v: u8) {
        self.write(nes, nes.cpu.s as u16 + 0x0100, v);
        nes.cpu.s -= 1;
    }

    fn push_stack_word(&mut self, nes: &mut Nes, v: u16) {
        self.push_stack(nes, (v >> 8) as u8);
        self.push_stack(nes, (v & 0xFF) as u8);
    }

    fn pull_stack(&mut self, nes: &mut Nes) -> u8 {
        nes.cpu.s += 1;
        self.read(nes, nes.cpu.s as u16 + 0x0100)
    }

    fn pull_stack_word(&mut self, nes: &mut Nes) -> u16 {
        self.pull_stack(nes) as u16 | (self.pull_stack(nes) as u16) << 8
    }
}

impl<T: EmuImpl> Emu for T {
    fn cpu_power_on(&mut self, nes: &mut Nes) {
        // https://wiki.nesdev.com/w/index.php/CPU_power_up_state

        // IRQ disabled
        nes.cpu.p = Status::from_bits_truncate(0x34);
        nes.cpu.a = 0x00;
        nes.cpu.x = 0x00;
        nes.cpu.y = 0x00;
        nes.cpu.s = 0xFD;
        // frame irq disabled
        self.write(nes, 0x4017, 0x00);
        // all channels disabled
        self.write(nes, 0x4015, 0x00);

        for a in 0x4000..=0x400F {
            self.write(nes, a, 0x00);
        }
        for a in 0x4010..=0x4013 {
            self.write(nes, a, 0x00);
        }
    }

    fn cpu_step(&mut self, nes: &mut Nes) {
        use addressing_mode::GetOperand;
        use decode::decode;
        use instruction::ExecuteInstruction;

        let op = self.fetch(nes);

        let (inst, mode) = decode(op);

        let operand = self.get_operand(nes, mode);
        self.execute(nes, (inst, mode), operand);
    }
}

fn page_crossed(a: u16, b: u16) -> bool {
    a.wrapping_add(b) & 0xFF00 != (b & 0xFF00)
}
