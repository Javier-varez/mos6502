
use crate::bus::Bus;
use crate::registers::Registers;
use crate::registers::StatusRegister;
use crate::addressing_modes::Operand;

#[derive(Copy, Clone)]
pub enum Instruction {
    Adc,
    And,
    Asl,
    Bcc,
    Bcs,
    Beq,
    Bit,
    Bmi,
    Bne,
    Bpl,
    Brk,
    Bvc,
    Bvs,
    Clc,
    Cld,
    Cli,
    Clv,
    Cmp,
    Cpx,
    Cpy,
    Dec,
    Dex,
    Dey,
    Eor,
    Inc,
    Inx,
    Iny,
    Jmp,
    Jsr,
    Lda,
    Ldx,
    Ldy,
    Lsr,
    Nop,
    Ora,
    Pha,
    Php,
    Pla,
    Plp,
    Rol,
    Ror,
    Rti,
    Rts,
    Sbc,
    Sec,
    Sed,
    Sei,
    Sta,
    Stx,
    Sty,
    Tax,
    Tay,
    Tsx,
    Txa,
    Txs,
    Tya,
}

impl Instruction {
    pub fn process<T: Bus>(&self, operand: Operand, bus: &mut T, regs: &mut Registers) {
        match self {
            // Logical operations
            Instruction::And => self.and(operand, bus, regs),
            Instruction::Ora => self.or_accumulator(operand, bus, regs),
            Instruction::Eor => self.exclusive_or(operand, bus, regs),
            // Comparison operations
            Instruction::Cmp => self.compare(operand, bus, regs, regs.accumulator),
            Instruction::Cpx => self.compare(operand, bus, regs, regs.x_index),
            Instruction::Cpy => self.compare(operand, bus, regs, regs.y_index),
            Instruction::Nop => { },

            // Bit manipulation operations
            Instruction::Bit => self.bit(operand, bus, regs),
            Instruction::Clc => { regs.status_reg.carry = false; },
            Instruction::Cld => { regs.status_reg.decimal_mode = false; },
            Instruction::Cli => { regs.status_reg.irq_disable = false; },
            Instruction::Clv => { regs.status_reg.overflow = false; },
            Instruction::Sec => { regs.status_reg.carry = true; },
            Instruction::Sed => { regs.status_reg.decimal_mode = true; },
            Instruction::Sei => { regs.status_reg.irq_disable = true; },

            // Arithmetic operations
            Instruction::Adc => self.add_with_carry(operand, bus, regs),
            Instruction::Sbc => self.subtract_with_carry(operand, bus, regs),

            // Increment/Decrement operations
            Instruction::Inc => self.increment_memory(operand, bus, regs),
            Instruction::Inx => self.increment_register(&mut regs.status_reg, &mut regs.x_index),
            Instruction::Iny => self.increment_register(&mut regs.status_reg, &mut regs.y_index),
            Instruction::Dec => self.decrement_memory(operand, bus, regs),
            Instruction::Dex => self.decrement_register(&mut regs.status_reg, &mut regs.x_index),
            Instruction::Dey => self.decrement_register(&mut regs.status_reg, &mut regs.x_index),

            // Data shifting instructions
            Instruction::Lsr => self.logical_shift_right(operand, bus, regs),
            Instruction::Asl => self.arithmetic_shift_left(operand, bus, regs),
            Instruction::Rol => self.rotate_left(operand, bus, regs),
            Instruction::Ror => self.rotate_right(operand, bus, regs),
            _ => {}
        }
    }

    fn and<T: Bus>(&self, operand: Operand, bus: &mut T, regs: &mut Registers) {
        let value = match operand {
            Operand::Value(val) => val,
            Operand::Addr(addr) => bus.read(addr),
            Operand::None => { panic!("AND requires an argument!"); }
        };

        regs.accumulator &= value;
        regs.status_reg.zero = regs.accumulator == 0;
        regs.status_reg.negative = (regs.accumulator & 0x80) != 0;
    }

    fn or_accumulator<T: Bus>(&self, operand: Operand, bus: &mut T, regs: &mut Registers) {
        let argument = match operand {
            Operand::Value(val) => val,
            Operand::Addr(addr) => bus.read(addr),
            Operand::None => { panic!("No argument for ORA!" ) }
        };

        regs.accumulator |= argument;
        regs.status_reg.negative = (regs.accumulator & 0x80) != 0;
        regs.status_reg.zero = regs.accumulator == 0x00;
    }

    fn exclusive_or<T: Bus>(&self, operand: Operand, bus: &mut T, regs: &mut Registers) {
        let argument = match operand {
            Operand::Value(val) => val,
            Operand::Addr(addr) => bus.read(addr),
            Operand::None => { panic!("No argument for ORA!" ) }
        };

        regs.accumulator ^= argument;
        regs.status_reg.negative = (regs.accumulator & 0x80) != 0;
        regs.status_reg.zero = regs.accumulator == 0x00;
    }

    fn compare<T: Bus>(&self, operand: Operand, bus: &mut T, regs: &mut Registers, val: u8) {
        let argument = match operand {
            Operand::Value(val) => val,
            Operand::Addr(addr) => bus.read(addr),
            Operand::None => { panic!("No argument for ORA!" ) }
        };

        regs.status_reg.negative = (val.wrapping_sub(argument) & 0x80) != 0;
        regs.status_reg.zero = val == argument;
        regs.status_reg.carry = val >= argument;
    }

    fn bit<T: Bus>(&self, operand: Operand, bus: &mut T, regs: &mut Registers) {
        let argument = match operand {
            Operand::Value(val) => val,
            Operand::Addr(addr) => bus.read(addr),
            Operand::None => { panic!("No argument for bit"); }
        };

        regs.status_reg.negative = (argument & 0x80) != 0;
        regs.status_reg.overflow = (argument & 0x40) != 0;
        regs.status_reg.zero = (argument & regs.accumulator) == 0;
    }

    fn add_with_carry<T: Bus>(&self, operand: Operand, bus: &mut T, regs: &mut Registers) {
        let value = match operand {
            Operand::Value(val) => val,
            Operand::Addr(addr) => bus.read(addr),
            Operand::None => { panic!("ADC requires an argument!"); }
        } as u16;

        if regs.status_reg.decimal_mode {
            panic!("Decimal mode not yet supported for ADC");
        }

        let mut acc = regs.accumulator as u16 + value;
        if regs.status_reg.carry {
            acc += 1;
        }

        regs.accumulator = acc as u8;
        regs.status_reg.carry = acc > 0xFF;
        regs.status_reg.zero = regs.accumulator == 0;
        regs.status_reg.negative = (regs.accumulator & 0x80) != 0;
    }

    fn subtract_with_carry<T: Bus>(&self, operand: Operand, bus: &mut T, regs: &mut Registers) {
        let mut value = match operand {
            Operand::Value(val) => val,
            Operand::Addr(addr) => bus.read(addr),
            Operand::None => { panic!("SBC requires an argument!"); }
        } as u16;

        if regs.status_reg.decimal_mode {
            panic!("Decimal mode not yet supported for SBC");
        }

        if !regs.status_reg.carry {
            value = value.wrapping_add(1);
        }

        let acc = regs.accumulator as u16 - value;

        regs.accumulator = acc as u8;
        regs.status_reg.carry = acc > 0xFF;
        regs.status_reg.zero = regs.accumulator == 0;
        regs.status_reg.negative = (regs.accumulator & 0x80) != 0;
    }

    fn increment_memory<T: Bus>(&self, operand: Operand, bus: &mut T, regs: &mut Registers) {
        if let Operand::Addr(addr) = operand {
            let mut value = bus.read(addr);
            value = value.wrapping_add(1);
            bus.write(addr, value);

            regs.status_reg.negative = (value & 0x80) != 0;
            regs.status_reg.zero = value == 0;
        } else {
            panic!("INC requires an address!")
        }
    }

    fn increment_register(&self, status_reg: &mut StatusRegister, reg: &mut u8) {
        *reg = reg.wrapping_add(1);

        status_reg.negative = (*reg & 0x80) != 0;
        status_reg.zero = *reg == 0;
    }

    fn decrement_memory<T: Bus>(&self, operand: Operand, bus: &mut T, regs: &mut Registers) {
        if let Operand::Addr(addr) = operand {
            let mut value = bus.read(addr);
            value = value.wrapping_sub(1);
            bus.write(addr, value);

            regs.status_reg.negative = (value & 0x80) != 0;
            regs.status_reg.zero = value == 0;
        } else {
            panic!("INC requires an address!")
        }
    }

    fn decrement_register(&self, status_reg: &mut StatusRegister, reg: &mut u8) {
        *reg = reg.wrapping_sub(1);

        status_reg.negative = (*reg & 0x80) != 0;
        status_reg.zero = *reg == 0;
    }

    fn logical_shift_right<T: Bus>(&self, operand: Operand, bus: &mut T, regs: &mut Registers) {
        let shift_op = | status_reg: &mut StatusRegister, value: &mut u8 | {
            status_reg.carry = (*value & 0x01) != 0;
            *value >>= 1;
            status_reg.zero = *value == 0;
            status_reg.negative = false;
        };

        match operand {
            Operand::Addr(addr) => {
                let mut val = bus.read(addr);
                shift_op(&mut regs.status_reg, &mut val);
                bus.write(addr, val);
            }
            Operand::Value(_val) => {
                let mut val = regs.accumulator;
                shift_op(&mut regs.status_reg, &mut val);
                regs.accumulator = val;
            },
            Operand::None => { panic!("Invalid operand None for LSR"); }
        };
    }

    fn arithmetic_shift_left<T: Bus>(&self, operand: Operand, bus: &mut T, regs: &mut Registers) {
        let shift_op = | value: &mut u8, regs: &mut Registers| {
            regs.status_reg.carry = (*value & 0x80) != 0;
            *value <<= 1;
            regs.status_reg.zero = *value == 0;
            regs.status_reg.negative = (*value & 0x80) != 0;
        };

        match operand {
            Operand::Addr(addr) => {
                let mut val = bus.read(addr);
                shift_op(&mut val, regs);
                bus.write(addr, val);
            }
            Operand::Value(_val) => {
                let mut val = regs.accumulator;
                shift_op(&mut val, regs);
                regs.accumulator = val;
            },
            Operand::None => { panic!("Invalid operand None for ASL"); }
        };
    }

    fn rotate_left<T: Bus>(&self, operand: Operand, bus: &mut T, regs: &mut Registers) {
        let shift_op = | value: &mut u8, regs: &mut Registers| {
            let carry = regs.status_reg.carry;
            regs.status_reg.carry = (*value & 0x80) != 0;

            *value <<= 1;
            if carry { *value |= 0x01; }

            regs.status_reg.zero = *value == 0;
            regs.status_reg.negative = (*value & 0x80) != 0;
        };

        match operand {
            Operand::Addr(addr) => {
                let mut val = bus.read(addr);
                shift_op(&mut val, regs);
                bus.write(addr, val);
            }
            Operand::Value(_val) => {
                let mut val = regs.accumulator;
                shift_op(&mut val, regs);
                regs.accumulator = val;
            },
            Operand::None => { panic!("Invalid operand None for ROL"); }
        };
    }

    fn rotate_right<T: Bus>(&self, operand: Operand, bus: &mut T, regs: &mut Registers) {
        let shift_op = | value: &mut u8, regs: &mut Registers| {
            let carry = regs.status_reg.carry;
            regs.status_reg.carry = (*value & 0x01) != 0;

            *value >>= 1;
            if carry { *value |= 0x80; }

            regs.status_reg.zero = *value == 0;
            regs.status_reg.negative = (*value & 0x80) != 0;
        };

        match operand {
            Operand::Addr(addr) => {
                let mut val = bus.read(addr);
                shift_op(&mut val, regs);
                bus.write(addr, val);
            }
            Operand::Value(_val) => {
                let mut val = regs.accumulator;
                shift_op(&mut val, regs);
                regs.accumulator = val;
            },
            Operand::None => { panic!("Invalid operand None for ROL"); }
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::bus::tests::DummyBus;

    #[test]
    fn adc_test() {
        let mut regs = Registers::new();
        let mut bus = DummyBus::new();
        let operand = Operand::Value(130);
        regs.accumulator = 10;

        let adc = Instruction::Adc;
        adc.process(operand, &mut bus, &mut regs);

        assert_eq!(regs.accumulator, 140);
        assert_eq!(regs.status_reg.carry, false);
        assert_eq!(regs.status_reg.negative, true);
        assert_eq!(regs.status_reg.zero, false);

        let operand = Operand::Addr(0x1234u16);
        bus.write(0x1234u16, 240);
        adc.process(operand, &mut bus, &mut regs);

        assert_eq!(regs.accumulator, 124);
        assert_eq!(regs.status_reg.carry, true);
        assert_eq!(regs.status_reg.negative, false);
        assert_eq!(regs.status_reg.zero, false);
    }

    #[test]
    #[should_panic]
    fn adc_test_none_argument() {
        let mut regs = Registers::new();
        let mut bus = DummyBus::new();
        let operand = Operand::None;
        let adc = Instruction::Adc;
        adc.process(operand, &mut bus, &mut regs);
    }

    #[test]
    fn and_test() {
        let mut regs = Registers::new();
        let mut bus = DummyBus::new();
        let operand = Operand::Value(0x72);
        regs.accumulator = 0xF0;
        regs.status_reg.negative = true;
        let and = Instruction::And;
        and.process(operand, &mut bus, &mut regs);

        assert_eq!(regs.accumulator, 0x70);
        assert_eq!(regs.status_reg.negative, false);
        assert_eq!(regs.status_reg.zero, false);
    }

    #[test]
    fn asl_test() {
        let mut regs = Registers::new();
        let mut bus = DummyBus::new();
        let operand = Operand::Value(0x72);
        regs.accumulator = 0xF0;
        regs.status_reg.negative = true;
        let and = Instruction::And;
        and.process(operand, &mut bus, &mut regs);

        assert_eq!(regs.accumulator, 0x70);
        assert_eq!(regs.status_reg.negative, false);
        assert_eq!(regs.status_reg.zero, false);
    }
}
