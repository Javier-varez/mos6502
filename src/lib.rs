
mod instruction;
mod bus;
mod opcodes;
mod addressing_modes;
mod registers;

pub use bus::Bus;
use opcodes::OPCODES;
use registers::Registers;

/// MOS 6502 Processor emulator
pub struct Cpu {
    registers: Registers
}

impl Cpu {
    ///
    /// Constructs a Cpu struct.
    ///
    pub fn new() -> Cpu {
        Cpu {
            registers: Registers::new(),
        }
    }

    ///
    /// Resets the processor, fetching the reset handler and jumping to it.
    ///
    pub fn reset<T: Bus>(&mut self, bus: &T) {
        // Restore original state
        *self = Cpu::new();

        // Jump to the reset handler.
        let low_byte : u16 = self.step_program_counter(bus).into();
        let high_byte : u16 = self.step_program_counter(bus).into();

        let reset_vector = low_byte | (high_byte << 8);
        self.registers.program_counter = reset_vector;
    }

    ///
    /// Runs a single instruction of the processor.
    ///
    /// # Example
    ///```
    ///    struct GndBus { }
    ///
    ///    impl mos6502::Bus for GndBus {
    ///        fn write(&mut self, _addr: u16, _value: u8) {
    ///          // Can't write in the Gnd bus, everything is tied to 0.
    ///        }
    ///        fn read(&self, _addr: u16) -> u8 {
    ///            0u8
    ///        }
    ///    }
    ///
    ///    let mut bus = GndBus {};
    ///    let mut mos6502 = mos6502::Cpu::new();
    ///    mos6502.reset(&mut bus);
    ///    mos6502.single_step(&mut bus);
    ///```
    ///
    pub fn single_step<T>(&mut self, bus: &mut T) where T: Bus {
        // Fetch opcode
        let (instruction, addressing_mode) = OPCODES[self.step_program_counter(bus) as usize].unwrap();
        let operand = addressing_mode.get_operand(bus, &mut self.registers);
        instruction.process(operand, bus, &mut self.registers);
    }

    /// Signals an interrupt (IRQB signal) to the core.
    pub fn signal_irq(&mut self) {
        self.registers.irq_active = true;
    }

    /// Signals a NMI Interrupt to the core.
    pub fn signal_nmi(&mut self) {
        self.registers.nmi_active = true;
    }

    ///
    /// Steps the program counter and returns the value at
    /// the current PC in the supplied Bus
    ///
    fn step_program_counter<T: Bus>(&mut self, bus: &T) -> u8 {
        let result = bus.read(self.registers.program_counter);
        self.registers.program_counter += 1;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct DummyBus {
        data: [u8; 0x10000],
    }

    impl DummyBus {
        fn new() -> DummyBus {
            DummyBus {
                data: [0u8; 0x10000]
            }
        }
    }

    impl<'a> Bus for DummyBus {
        fn write(&mut self, _addr: u16, _value: u8) { }

        fn read(&self, addr: u16) -> u8 {
            self.data[addr as usize]
        }
    }

    #[test]
    fn test_reset() {
        let mut cpu = Cpu::new();
        let mut bus = DummyBus::new();
        bus.data[0xFFFC] = 0x12;
        bus.data[0xFFFD] = 0x34;
        cpu.reset(&bus);

        assert_eq!(cpu.registers.program_counter, 0x3412);
        assert_eq!(cpu.registers.accumulator, 0);
    }
}