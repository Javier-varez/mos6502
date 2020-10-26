
use crate::bus::Bus;
use crate::registers::Registers;
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
            Instruction::Adc => self.add_with_carry(operand, bus, regs),
            Instruction::And => self.and(operand, bus, regs),
            _ => {}
        }
    }

    fn add_with_carry<T: Bus>(&self, operand: Operand, bus: &mut T, regs: &mut Registers) {
        let value = match operand {
            Operand::Value(val) => val,
            Operand::Addr(addr) => bus.read(addr),
            Operand::None => { panic!("ADC requires an argument!"); }
        } as u16;

        let mut acc = regs.accumulator as u16 + value;
        if regs.status_reg.carry {
            acc += 1;
        }

        regs.accumulator = acc as u8;
        regs.status_reg.carry = acc > 0xFF;
        regs.status_reg.zero = regs.accumulator == 0;
        regs.status_reg.negative = (regs.accumulator & 0x80) != 0;
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
}
