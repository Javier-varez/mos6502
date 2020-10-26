
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
}

pub struct Registers {
    pub stack_pointer: u8,
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
            stack_pointer: 0xFF,
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
