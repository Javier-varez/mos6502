
/// Interface to an interconnect bus
/// Transport interface between multiple components in the bus
pub trait Bus {
    /// Writes a byte to the given address in the bus
    fn write(&mut self, addr: u16, value: u8);
    /// Reads a byte from the given address in the bus
    fn read(&self, addr: u16) -> u8;
}

#[cfg(test)]
pub mod tests {
    use super::*;
    pub struct DummyBus {
        mem: [u8; 65536]
    }

    impl DummyBus {
        pub fn new() -> DummyBus {
            DummyBus {
                mem: [0u8; 65536]
            }
        }
    }

    impl Bus for DummyBus {
        fn read(&self, addr: u16) -> u8 {
            self.mem[addr as usize]
        }

        fn write(&mut self, addr: u16, value: u8) {
            self.mem[addr as usize] = value;
        }
    }
}