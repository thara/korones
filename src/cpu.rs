use crate::nes::Nes;

#[derive(Debug)]
#[allow(dead_code)]
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

    // CPU cycles
    cycles: u128,
    // CPU internal RAM
    ram: [u8; 0x2000],
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
    #[allow(dead_code)]
    fn set_zn(&mut self, v: u8) {
        self.set(Self::Z, v == 0);
        self.set(Self::N, (v >> 7) & 1 == 1);
    }
}

pub(super) trait CpuEmu {
    type Bus: CpuBus;

    fn on_cpu_tick(nes: &mut Nes);

    fn cpu_step(nes: &mut Nes) {
        todo!();
    }

    fn read(nes: &mut Nes, addr: u16) -> u8 {
        let value = match addr {
            0x0000..=0x1FFF => nes.cpu.ram[addr as usize].into(),
            0x2000..=0x3FFF => Self::Bus::read_ppu_reg(nes, addr),
            0x4000..=0x4013 | 0x4015 => Self::Bus::read_apu_reg(nes, addr),
            0x4016 | 0x4017 => Self::Bus::read_controller(nes, addr),
            0x4020..=0xFFFF => Self::Bus::read_mapper(nes, addr),
            _ => 0u8.into(),
        };
        Self::tick(nes);
        value
    }

    fn write(nes: &mut Nes, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1FFF => nes.cpu.ram[addr as usize] = value,
            0x2000..=0x3FFF => Self::Bus::write_ppu_reg(nes, addr, value),
            0x4000..=0x4013 | 0x4015 => Self::Bus::write_apu_reg(nes, addr, value),
            0x4016 => Self::Bus::write_controller(nes, addr, value),
            0x4017 => {
                Self::Bus::write_controller(nes, addr, value);
                Self::Bus::write_apu_reg(nes, addr, value);
            }
            0x4020..=0xFFFF => Self::Bus::write_mapper(nes, addr, value),
            _ => {
                //NOP
            }
        }
        Self::tick(nes);
    }

    fn tick(nes: &mut Nes) {
        Self::on_cpu_tick(nes);
        nes.cpu.cycles += 1;
    }
}

pub(super) trait CpuBus {
    fn read_ppu_reg(nes: &mut Nes, addr: u16) -> u8;
    fn write_ppu_reg(nes: &mut Nes, addr: u16, value: u8);

    fn read_apu_reg(nes: &mut Nes, addr: u16) -> u8;
    fn write_apu_reg(nes: &mut Nes, addr: u16, value: u8);

    fn read_controller(nes: &mut Nes, addr: u16) -> u8;
    fn write_controller(nes: &mut Nes, addr: u16, value: u8);

    fn read_mapper(nes: &mut Nes, addr: u16) -> u8;
    fn write_mapper(nes: &mut Nes, addr: u16, value: u8);
}
