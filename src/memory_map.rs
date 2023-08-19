use crate::nes::Nes;

pub(crate) trait MemoryMap<T> {
    fn read(state: &mut T, addr: u16) -> u8;
    fn write(state: &mut T, addr: u16, value: u8);

    fn read_word(state: &mut T, addr: u16) -> u16 {
        Self::read(state, addr) as u16 | ((Self::read(state, addr + 1) as u16) << 8)
    }
}
