
use crate::bus::Bus;
use crate::registers::Registers;

#[derive(Copy, Clone)]
pub enum AddressingMode {
    Accumulator,
    Absolute,
    AbsoluteXIndexed,
    AbsoluteYIndexed,
    Immediate,
    Implied,
    Indirect,
    XIndexedIndirect,
    IndirectYIndexed,
    Relative,
    Zeropage,
    ZeropageXIndexed,
    ZeropageYIndexed,
}

#[derive(Debug, PartialEq)]
pub enum Operand {
    Addr(u16),
    Value(u8),
    None
}

impl AddressingMode {
    pub fn get_operand<T: Bus>(&self, bus: &mut T, regs: &mut Registers) -> Operand {
        match self {
            AddressingMode::Accumulator => {
                Operand::Value(regs.accumulator)
            },
            AddressingMode::Absolute => {
                Operand::Addr(self.get_absolute_address(bus, regs))
            },
            AddressingMode::AbsoluteXIndexed => {
                Operand::Addr(self.get_absolute_address(bus, regs).wrapping_add(regs.x_index as u16))
            },
            AddressingMode::AbsoluteYIndexed => {
                Operand::Addr(self.get_absolute_address(bus, regs).wrapping_add(regs.y_index as u16))
            },
            AddressingMode::Immediate => {
                Operand::Value(self.get_immediate_value(bus, regs))
            },
            AddressingMode::Implied => {
                Operand::None
            },
            AddressingMode::Indirect => {
                Operand::Addr(self.get_indirect_address(bus, regs))
            },
            AddressingMode::XIndexedIndirect => {
                Operand::Addr(self.get_x_indexed_indirect_address(bus, regs))
            },
            AddressingMode::IndirectYIndexed => {
                Operand::Addr(self.get_indirect_y_indexed_address(bus, regs))
            },
            AddressingMode::Relative => {
                Operand::Addr(self.get_relative_addr(bus, regs))
            },
            AddressingMode::Zeropage => {
                Operand::Addr(self.get_zeropage_addr(bus, regs))
            },
            AddressingMode::ZeropageXIndexed => {
                Operand::Addr(self.get_zeropage_x_indexed_addr(bus, regs))
            },
            AddressingMode::ZeropageYIndexed => {
                Operand::Addr(self.get_zeropage_y_indexed_addr(bus, regs))
            }
        }
    }

    fn get_absolute_address<T: Bus>(&self, bus: &mut T, regs: &mut Registers) -> u16 {
        let lo : u16 = bus.read(regs.program_counter).into();
        regs.program_counter += 1;
        let hi : u16 = bus.read(regs.program_counter).into();
        regs.program_counter += 1;
        hi << 8 | lo
    }

    fn get_immediate_value<T: Bus>(&self, bus: &mut T, regs: &mut Registers) -> u8 {
        let val = bus.read(regs.program_counter);
        regs.program_counter += 1;
        val
    }

    fn get_indirect_address<T: Bus>(&self, bus: &mut T, regs: &mut Registers) -> u16{
        let lo : u16 = bus.read(regs.program_counter).into();
        regs.program_counter += 1;
        let hi : u16 = bus.read(regs.program_counter).into();
        regs.program_counter += 1;

        let ptr: u16 = hi << 8 | lo;
        // The pointer wraps around in a page boundary
        let ptr_plus_1 = match lo {
            0xff => hi << 8,
            _ => ptr + 1
        };
        let lo: u16 = bus.read(ptr).into();
        let hi: u16 = bus.read(ptr_plus_1).into();
        hi << 8 | lo
    }

    fn get_x_indexed_indirect_address<T: Bus>(&self, bus: &mut T, regs: &mut Registers) -> u16 {
        let lo : u16 = bus.read(regs.program_counter).into();
        regs.program_counter += 1;
        let hi : u16 = bus.read(regs.program_counter).into();
        regs.program_counter += 1;

        let ptr: u16 = (hi << 8 | lo) + regs.x_index as u16;
        // The pointer wraps around in a page boundary
        let ptr_plus_1 = match lo {
            0xff => hi << 8,
            _ => ptr + 1
        };
        let lo: u16 = bus.read(ptr).into();
        let hi: u16 = bus.read(ptr_plus_1).into();
        hi << 8 | lo
    }

    fn get_indirect_y_indexed_address<T: Bus>(&self, bus: &mut T, regs: &mut Registers) -> u16 {
        self.get_indirect_address(bus, regs) + regs.y_index as u16
    }

    fn get_relative_addr<T: Bus>(&self, bus: &mut T, regs: &mut Registers) -> u16 {
        let relative_immediate = bus.read(regs.program_counter) as i8;
        regs.program_counter += 1;
        regs.program_counter.wrapping_add(relative_immediate as u16)
    }

    fn get_zeropage_addr<T: Bus>(&self, bus: &mut T, regs: &mut Registers) -> u16 {
        let zeropage_addr = bus.read(regs.program_counter) as u16;
        regs.program_counter += 1;
        zeropage_addr
    }

    fn get_zeropage_x_indexed_addr<T: Bus>(&self, bus: &mut T, regs: &mut Registers) -> u16 {
        let zeropage_addr = self.get_zeropage_addr(bus, regs) as u8;
        let addr = zeropage_addr.wrapping_add(regs.x_index);
        addr as u16
    }

    fn get_zeropage_y_indexed_addr<T: Bus>(&self, bus: &mut T, regs: &mut Registers) -> u16 {
        let zeropage_addr = self.get_zeropage_addr(bus, regs) as u8;
        let addr = zeropage_addr.wrapping_add(regs.y_index);
        addr as u16
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::bus::tests::DummyBus;

    #[test]
    fn test_accumulator_addressing_mode() {
        let am = AddressingMode::Accumulator;
        let mut bus = DummyBus::new();
        let mut regs = Registers::new();
        regs.accumulator = 123;

        assert_eq!(am.get_operand(&mut bus, &mut regs), Operand::Value(123));
    }

    #[test]
    fn test_absolute_addressing_mode() {
        let am = AddressingMode::Absolute;
        let mut bus = DummyBus::new();
        let mut regs = Registers::new();
        regs.program_counter = 0x1234;
        bus.write(0x1234, 0x23u8);
        bus.write(0x1235, 0x34u8);

        assert_eq!(am.get_operand(&mut bus, &mut regs), Operand::Addr(0x3423));
    }

    #[test]
    fn test_absolute_x_index_addressing_mode() {
        let am = AddressingMode::AbsoluteXIndexed;
        let mut bus = DummyBus::new();
        let mut regs = Registers::new();
        regs.program_counter = 0x1234;
        regs.x_index = 12;
        bus.write(0x1234, 0x23u8);
        bus.write(0x1235, 0x34u8);

        assert_eq!(am.get_operand(&mut bus, &mut regs), Operand::Addr(0x342F));
    }

    #[test]
    fn test_absolute_y_index_addressing_mode() {
        let am = AddressingMode::AbsoluteYIndexed;
        let mut bus = DummyBus::new();
        let mut regs = Registers::new();
        regs.program_counter = 0x1234;
        regs.y_index = 12;
        bus.write(0x1234, 0x23u8);
        bus.write(0x1235, 0x34u8);

        assert_eq!(am.get_operand(&mut bus, &mut regs), Operand::Addr(0x342F));
    }

    #[test]
    fn test_immediate_addressing_mode() {
        let am = AddressingMode::Immediate;
        let mut bus = DummyBus::new();
        let mut regs = Registers::new();
        regs.program_counter = 0x1234;
        bus.write(0x1234, 0x23u8);

        assert_eq!(am.get_operand(&mut bus, &mut regs), Operand::Value(0x23));
    }

    #[test]
    fn test_implied_addressing_mode() {
        let am = AddressingMode::Implied;
        let mut bus = DummyBus::new();
        let mut regs = Registers::new();

        assert_eq!(am.get_operand(&mut bus, &mut regs), Operand::None);
    }

    #[test]
    fn test_indirect_addressing_mode() {
        let am = AddressingMode::Indirect;
        let mut bus = DummyBus::new();
        let mut regs = Registers::new();
        regs.program_counter = 0x1234;
        bus.write(0x1234, 0x23);
        bus.write(0x1235, 0x64);
        bus.write(0x6423, 0xFF);
        bus.write(0x6424, 0x2F);

        assert_eq!(am.get_operand(&mut bus, &mut regs), Operand::Addr(0x2FFF));
    }

    #[test]
    fn test_x_indexed_indirect_addressing_mode() {
        let am = AddressingMode::XIndexedIndirect;
        let mut bus = DummyBus::new();
        let mut regs = Registers::new();
        regs.program_counter = 0x1234;
        regs.x_index = 0x20;
        bus.write(0x1234, 0x23);
        bus.write(0x1235, 0x64);
        bus.write(0x6443, 0xFF);
        bus.write(0x6444, 0x2F);

        assert_eq!(am.get_operand(&mut bus, &mut regs), Operand::Addr(0x2FFF));
    }

    #[test]
    fn test_indirect_y_indexed_addressing_mode() {
        let am = AddressingMode::IndirectYIndexed;
        let mut bus = DummyBus::new();
        let mut regs = Registers::new();
        regs.program_counter = 0x1234;
        regs.y_index = 0x20;
        bus.write(0x1234, 0x23);
        bus.write(0x1235, 0x64);
        bus.write(0x6423, 0x30);
        bus.write(0x6424, 0x2F);

        assert_eq!(am.get_operand(&mut bus, &mut regs), Operand::Addr(0x2F50));
    }

    #[test]
    fn test_relative_addressing_mode() {
        let am = AddressingMode::Relative;
        let mut bus = DummyBus::new();
        let mut regs = Registers::new();
        regs.program_counter = 0x1234;
        bus.write(0x1234, 0x23);
        bus.write(0x1235, -0x10i8 as u8);

        assert_eq!(am.get_operand(&mut bus, &mut regs), Operand::Addr(0x1258));
        assert_eq!(am.get_operand(&mut bus, &mut regs), Operand::Addr(0x1226));
    }

    #[test]
    fn test_zeropage_addressing_mode() {
        let am = AddressingMode::Zeropage;
        let mut bus = DummyBus::new();
        let mut regs = Registers::new();
        regs.program_counter = 0x1234;
        bus.write(0x1234, 0x56);
        bus.write(0x1235, 0x52);

        assert_eq!(am.get_operand(&mut bus, &mut regs), Operand::Addr(0x56));
    }

    #[test]
    fn test_zeropage_x_indexed_addressing_mode() {
        let am = AddressingMode::ZeropageXIndexed;
        let mut bus = DummyBus::new();
        let mut regs = Registers::new();
        regs.program_counter = 0x1234;
        regs.x_index = 0x80;
        bus.write(0x1234, 0x56);
        bus.write(0x1235, 0x52);

        assert_eq!(am.get_operand(&mut bus, &mut regs), Operand::Addr(0xD6));
    }

    #[test]
    fn test_zeropage_y_indexed_addressing_mode() {
        let am = AddressingMode::ZeropageYIndexed;
        let mut bus = DummyBus::new();
        let mut regs = Registers::new();
        regs.program_counter = 0x1234;
        regs.y_index = 0x80;
        bus.write(0x1234, 0x56);
        bus.write(0x1235, 0x52);

        assert_eq!(am.get_operand(&mut bus, &mut regs), Operand::Addr(0xD6));
    }
}
