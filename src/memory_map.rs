use crate::nes::Nes;

pub(crate) trait MemoryMap {
    fn read(nes: &mut Nes, addr: u16) -> u8;
    fn write(nes: &mut Nes, addr: u16, value: u8);

    fn read_word(nes: &mut Nes, addr: u16) -> u16 {
        Self::read(nes, addr) as u16 | ((Self::read(nes, addr + 1) as u16) << 8)
    }
}
