use crate::cpu::{CpuBus, CpuEmu};
use crate::nes::Nes;

pub(crate) struct Emu {}

impl CpuEmu for Emu {
    type Bus = Bus;

    fn on_cpu_tick(nes: &mut Nes) {}
}

pub(crate) struct Bus {}

//TODO
impl CpuBus for Bus {
    fn read_ppu_reg(nes: &mut Nes, addr: u16) -> u8 {
        todo!();
    }
    fn write_ppu_reg(nes: &mut Nes, addr: u16, value: u8) {
        todo!();
    }

    fn read_apu_reg(nes: &mut Nes, addr: u16) -> u8 {
        todo!();
    }
    fn write_apu_reg(nes: &mut Nes, addr: u16, value: u8) {
        todo!();
    }

    fn read_controller(nes: &mut Nes, addr: u16) -> u8 {
        todo!();
    }
    fn write_controller(nes: &mut Nes, addr: u16, value: u8) {
        todo!();
    }

    fn read_mapper(nes: &mut Nes, addr: u16) -> u8 {
        todo!();
    }
    fn write_mapper(nes: &mut Nes, addr: u16, value: u8) {
        todo!();
    }
}
