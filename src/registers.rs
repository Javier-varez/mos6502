use crate::bus::Bus;

pub struct StatusRegister {
    pub carry: bool,
    pub zero: bool,
    pub irq_disable: bool,
    pub decimal_mode: bool,
    pub brk_command: bool,
    pub overflow: bool,
    pub negative: bool,
}

impl StatusRegister {
    fn new() -> StatusRegister {
        StatusRegister {
            carry: false,
            zero: false,
            irq_disable: false,
            decimal_mode: false,
            brk_command: false,
            overflow: false,
            negative: false
        }
    }

    pub fn get(&self) -> u8 {
        let mut value = 0x00;
        if self.negative { value |= 0x80; }
        if self.overflow { value |= 0x40; }
        if self.brk_command { value |= 0x10; }
        if self.decimal_mode { value |= 0x08; }
        if self.irq_disable { value |= 0x04; }
        if self.zero { value |= 0x02; }
        if self.carry { value |= 0x01; }
        value
    }

    pub fn set(&mut self, value: u8) {
        self.negative = (value & 0x80) != 0;
        self.overflow = (value & 0x40) != 0;
        self.brk_command = (value & 0x10) != 0;
        self.decimal_mode = (value & 0x08) != 0;
        self.irq_disable = (value & 0x04) != 0;
        self.zero = (value & 0x02) != 0;
        self.carry = (value & 0x01) != 0;
    }
}

pub struct Stack {
    pointer: u8
}

impl Stack {
    fn new() -> Self {
        Self {
            pointer: 0xFF
        }
    }

    pub fn push<T: Bus>(&mut self, value: u8, bus: &mut T) {
        let addr = 0x100u16 | self.pointer as u16;
        bus.write(addr, value);
        self.pointer = self.pointer.wrapping_sub(1);
    }

    pub fn pop<T: Bus>(&mut self, bus: &T) -> u8 {
        self.pointer = self.pointer.wrapping_add(1);
        let addr = 0x100u16 | self.pointer as u16;
        bus.read(addr)
    }

    pub fn get(&self) -> u8 {
        self.pointer
    }

    pub fn set(&mut self, value: u8) {
        self.pointer = value;
    }
}

pub struct Registers {
    pub stack: Stack,
    pub x_index: u8,
    pub y_index: u8,
    pub program_counter: u16,
    pub accumulator: u8,
    pub status_reg: StatusRegister,
    pub irq_active: bool,
    pub nmi_active: bool,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            stack: Stack::new(),
            x_index: 0x00,
            y_index: 0x00,
            program_counter: 0xFFFC,
            accumulator: 0,
            status_reg: StatusRegister::new(),
            irq_active: false,
            nmi_active: false,
        }
    }
}
