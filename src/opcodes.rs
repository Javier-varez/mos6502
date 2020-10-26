
use crate::instruction::Instruction;
use crate::addressing_modes::AddressingMode;

pub static OPCODES : [Option<(Instruction, AddressingMode)>; 256 ] = [
    Some((Instruction::Brk, AddressingMode::Implied)),                // 0x00
    Some((Instruction::Ora, AddressingMode::XIndexedIndirect)),       // 0x01
    None,                                                             // 0x02
    None,                                                             // 0x03
    None,                                                             // 0x04
    Some((Instruction::Ora, AddressingMode::Zeropage)),               // 0x05
    Some((Instruction::Asl, AddressingMode::Zeropage)),               // 0x06
    None,                                                             // 0x07
    Some((Instruction::Php, AddressingMode::Implied)),                // 0x08
    Some((Instruction::Ora, AddressingMode::Immediate)),              // 0x09
    Some((Instruction::Asl, AddressingMode::Accumulator)),            // 0x0A
    None,                                                             // 0x0B
    None,                                                             // 0x0C
    Some((Instruction::Ora, AddressingMode::Absolute)),               // 0x0D
    Some((Instruction::Asl, AddressingMode::Absolute)),               // 0x0E
    None,                                                             // 0x0F
    Some((Instruction::Bpl, AddressingMode::Relative)),               // 0x10
    Some((Instruction::Ora, AddressingMode::AbsoluteYIndexed)),       // 0x11
    None,                                                             // 0x12
    None,                                                             // 0x13
    None,                                                             // 0x14
    Some((Instruction::Ora, AddressingMode::ZeropageXIndexed)),       // 0x15
    Some((Instruction::Asl, AddressingMode::ZeropageXIndexed)),       // 0x16
    None,                                                             // 0x17
    Some((Instruction::Clc, AddressingMode::Implied)),                // 0x18
    Some((Instruction::Ora, AddressingMode::AbsoluteYIndexed)),       // 0x19
    None,                                                             // 0x1A
    None,                                                             // 0x1B
    None,                                                             // 0x1C
    Some((Instruction::Ora, AddressingMode::AbsoluteXIndexed)),       // 0x1D
    Some((Instruction::Asl, AddressingMode::AbsoluteXIndexed)),       // 0x1E
    None,                                                             // 0x1F
    Some((Instruction::Jsr, AddressingMode::Absolute)),               // 0x20
    Some((Instruction::And, AddressingMode::XIndexedIndirect)),       // 0x21
    None,                                                             // 0x22
    None,                                                             // 0x23
    Some((Instruction::Bit, AddressingMode::Zeropage)),               // 0x24
    Some((Instruction::And, AddressingMode::Zeropage)),               // 0x25
    Some((Instruction::Rol, AddressingMode::Zeropage)),               // 0x26
    None,                                                             // 0x27
    Some((Instruction::Plp, AddressingMode::Implied)),                // 0x28
    Some((Instruction::And, AddressingMode::Immediate)),              // 0x29
    Some((Instruction::Rol, AddressingMode::Accumulator)),            // 0x2A
    None,                                                             // 0x2B
    Some((Instruction::Bit, AddressingMode::Absolute)),               // 0x2C
    Some((Instruction::And, AddressingMode::Absolute)),               // 0x2D
    Some((Instruction::Rol, AddressingMode::Absolute)),               // 0x2E
    None,                                                             // 0x2F
    Some((Instruction::Bmi, AddressingMode::Relative)),               // 0x30
    Some((Instruction::And, AddressingMode::IndirectYIndexed)),       // 0x31
    None,                                                             // 0x32
    None,                                                             // 0x33
    None,                                                             // 0x34
    Some((Instruction::And, AddressingMode::ZeropageXIndexed)),       // 0x35
    Some((Instruction::Rol, AddressingMode::ZeropageXIndexed)),       // 0x36
    None,                                                             // 0x37
    Some((Instruction::Sec, AddressingMode::Implied)),                // 0x38
    Some((Instruction::And, AddressingMode::AbsoluteYIndexed)),       // 0x39
    None,                                                             // 0x3A
    None,                                                             // 0x3B
    None,                                                             // 0x3C
    Some((Instruction::And, AddressingMode::AbsoluteXIndexed)),       // 0x3D
    Some((Instruction::Rol, AddressingMode::AbsoluteXIndexed)),       // 0x3E
    None,                                                             // 0x3F
    Some((Instruction::Rti, AddressingMode::Implied)),                // 0x40
    Some((Instruction::Eor, AddressingMode::XIndexedIndirect)),       // 0x41
    None,                                                             // 0x42
    None,                                                             // 0x43
    None,                                                             // 0x44
    Some((Instruction::Eor, AddressingMode::Zeropage)),               // 0x45
    Some((Instruction::Lsr, AddressingMode::Zeropage)),               // 0x46
    None,                                                             // 0x47
    Some((Instruction::Pha, AddressingMode::Implied)),                // 0x48
    Some((Instruction::Eor, AddressingMode::Immediate)),              // 0x49
    Some((Instruction::Lsr, AddressingMode::Accumulator)),            // 0x4A
    None,                                                             // 0x4B
    Some((Instruction::Jmp, AddressingMode::Absolute)),               // 0x4C
    Some((Instruction::Eor, AddressingMode::Absolute)),               // 0x4D
    Some((Instruction::Lsr, AddressingMode::Absolute)),               // 0x4E
    None,                                                             // 0x4F
    Some((Instruction::Bvc, AddressingMode::Relative)),               // 0x50
    Some((Instruction::Eor, AddressingMode::IndirectYIndexed)),       // 0x51
    None,                                                             // 0x52
    None,                                                             // 0x53
    None,                                                             // 0x54
    Some((Instruction::Eor, AddressingMode::ZeropageXIndexed)),       // 0x55
    Some((Instruction::Lsr, AddressingMode::ZeropageXIndexed)),       // 0x56
    None,                                                             // 0x57
    Some((Instruction::Cli, AddressingMode::Implied)),                // 0x58
    Some((Instruction::Eor, AddressingMode::AbsoluteYIndexed)),       // 0x59
    None,                                                             // 0x5A
    None,                                                             // 0x5B
    None,                                                             // 0x5C
    Some((Instruction::Eor, AddressingMode::AbsoluteXIndexed)),       // 0x5D
    Some((Instruction::Lsr, AddressingMode::AbsoluteXIndexed)),       // 0x5E
    None,                                                             // 0x5F
    Some((Instruction::Rts, AddressingMode::Implied)),                // 0x60
    Some((Instruction::Adc, AddressingMode::XIndexedIndirect)),       // 0x61
    None,                                                             // 0x62
    None,                                                             // 0x63
    None,                                                             // 0x64
    Some((Instruction::Adc, AddressingMode::Zeropage)),               // 0x65
    Some((Instruction::Ror, AddressingMode::Zeropage)),               // 0x66
    None,                                                             // 0x67
    Some((Instruction::Pla, AddressingMode::Implied)),                // 0x68
    Some((Instruction::Adc, AddressingMode::Immediate)),              // 0x69
    Some((Instruction::Ror, AddressingMode::Accumulator)),            // 0x6A
    None,                                                             // 0x6B
    Some((Instruction::Jmp, AddressingMode::Indirect)),               // 0x6C
    Some((Instruction::Adc, AddressingMode::Absolute)),               // 0x6D
    Some((Instruction::Ror, AddressingMode::Absolute)),               // 0x6E
    None,                                                             // 0x6F
    Some((Instruction::Bvs, AddressingMode::Relative)),               // 0x70
    Some((Instruction::Adc, AddressingMode::IndirectYIndexed)),       // 0x71
    None,                                                             // 0x72
    None,                                                             // 0x73
    None,                                                             // 0x74
    Some((Instruction::Adc, AddressingMode::ZeropageXIndexed)),       // 0x75
    Some((Instruction::Ror, AddressingMode::ZeropageXIndexed)),       // 0x76
    None,                                                             // 0x77
    Some((Instruction::Sei, AddressingMode::Implied)),                // 0x78
    Some((Instruction::Adc, AddressingMode::AbsoluteYIndexed)),       // 0x79
    None,                                                             // 0x7A
    None,                                                             // 0x7B
    None,                                                             // 0x7C
    Some((Instruction::Adc, AddressingMode::AbsoluteXIndexed)),       // 0x7D
    Some((Instruction::Ror, AddressingMode::AbsoluteXIndexed)),       // 0x7E
    None,                                                             // 0x7F
    None,                                                             // 0x80
    Some((Instruction::Sta, AddressingMode::IndirectYIndexed)),       // 0x81
    None,                                                             // 0x82
    None,                                                             // 0x83
    Some((Instruction::Sty, AddressingMode::Zeropage)),               // 0x84
    Some((Instruction::Sta, AddressingMode::Zeropage)),               // 0x85
    Some((Instruction::Stx, AddressingMode::Zeropage)),               // 0x86
    None,                                                             // 0x87
    Some((Instruction::Dey, AddressingMode::Implied)),                // 0x88
    None,                                                             // 0x89
    Some((Instruction::Txa, AddressingMode::Implied)),                // 0x8A
    None,                                                             // 0x8B
    Some((Instruction::Sty, AddressingMode::Absolute)),               // 0x8C
    Some((Instruction::Sta, AddressingMode::Absolute)),               // 0x8D
    Some((Instruction::Stx, AddressingMode::Absolute)),               // 0x8E
    None,                                                             // 0x8F
    Some((Instruction::Bcc, AddressingMode::Relative)),               // 0x90
    Some((Instruction::Sta, AddressingMode::IndirectYIndexed)),       // 0x91
    None,                                                             // 0x92
    None,                                                             // 0x93
    Some((Instruction::Sty, AddressingMode::ZeropageXIndexed)),       // 0x94
    Some((Instruction::Sta, AddressingMode::ZeropageXIndexed)),       // 0x95
    Some((Instruction::Stx, AddressingMode::ZeropageXIndexed)),       // 0x96
    None,                                                             // 0x97
    Some((Instruction::Tya, AddressingMode::Implied)),                // 0x98
    Some((Instruction::Sta, AddressingMode::AbsoluteYIndexed)),       // 0x99
    Some((Instruction::Txs, AddressingMode::Implied)),                // 0x9A
    None,                                                             // 0x9B
    None,                                                             // 0x9C
    Some((Instruction::Sta, AddressingMode::AbsoluteXIndexed)),       // 0x9D
    None,                                                             // 0x9E
    None,                                                             // 0x9F
    Some((Instruction::Ldy, AddressingMode::Immediate)),              // 0xA0
    Some((Instruction::Lda, AddressingMode::XIndexedIndirect)),       // 0xA1
    Some((Instruction::Ldx, AddressingMode::Immediate)),              // 0xA2
    None,                                                             // 0xA3
    Some((Instruction::Ldy, AddressingMode::Zeropage)),               // 0xA4
    Some((Instruction::Lda, AddressingMode::Zeropage)),               // 0xA5
    Some((Instruction::Ldx, AddressingMode::Zeropage)),               // 0xA6
    None,                                                             // 0xA7
    Some((Instruction::Tay, AddressingMode::Implied)),                // 0xA8
    Some((Instruction::Lda, AddressingMode::Immediate)),              // 0xA9
    Some((Instruction::Tax, AddressingMode::Implied)),                // 0xAA
    None,                                                             // 0xAB
    Some((Instruction::Ldy, AddressingMode::Absolute)),               // 0xAC
    Some((Instruction::Lda, AddressingMode::Absolute)),               // 0xAD
    Some((Instruction::Ldx, AddressingMode::Absolute)),               // 0xAE
    None,                                                             // 0xAF
    Some((Instruction::Bcs, AddressingMode::Relative)),               // 0xB0
    Some((Instruction::Lda, AddressingMode::IndirectYIndexed)),       // 0xB1
    None,                                                             // 0xB2
    None,                                                             // 0xB3
    Some((Instruction::Ldy, AddressingMode::ZeropageXIndexed)),       // 0xB4
    Some((Instruction::Lda, AddressingMode::ZeropageXIndexed)),       // 0xB5
    Some((Instruction::Ldx, AddressingMode::ZeropageYIndexed)),       // 0xB6
    None,                                                             // 0xB7
    Some((Instruction::Clv, AddressingMode::Implied)),                // 0xB8
    Some((Instruction::Lda, AddressingMode::AbsoluteYIndexed)),       // 0xB9
    Some((Instruction::Tsx, AddressingMode::Implied)),                // 0xBA
    None,                                                             // 0xBB
    Some((Instruction::Ldy, AddressingMode::AbsoluteXIndexed)),       // 0xBC
    Some((Instruction::Lda, AddressingMode::AbsoluteXIndexed)),       // 0xBD
    Some((Instruction::Ldx, AddressingMode::AbsoluteYIndexed)),       // 0xBE
    None,                                                             // 0xBF
    Some((Instruction::Cpy, AddressingMode::Immediate)),              // 0xC0
    Some((Instruction::Cmp, AddressingMode::XIndexedIndirect)),       // 0xC1
    None,                                                             // 0xC2
    None,                                                             // 0xC3
    Some((Instruction::Cpy, AddressingMode::Zeropage)),               // 0xC4
    Some((Instruction::Cmp, AddressingMode::Zeropage)),               // 0xC5
    Some((Instruction::Dec, AddressingMode::Zeropage)),               // 0xC6
    None,                                                             // 0xC7
    Some((Instruction::Iny, AddressingMode::Implied)),                // 0xC8
    Some((Instruction::Cmp, AddressingMode::Immediate)),              // 0xC9
    Some((Instruction::Dex, AddressingMode::Implied)),                // 0xCA
    None,                                                             // 0xCB
    Some((Instruction::Cpy, AddressingMode::Absolute)),               // 0xCC
    Some((Instruction::Cmp, AddressingMode::Absolute)),               // 0xCD
    Some((Instruction::Dec, AddressingMode::Absolute)),               // 0xCE
    None,                                                             // 0xCF
    Some((Instruction::Bne, AddressingMode::Relative)),               // 0xD0
    Some((Instruction::Cmp, AddressingMode::IndirectYIndexed)),       // 0xD1
    None,                                                             // 0xD2
    None,                                                             // 0xD3
    None,                                                             // 0xD4
    Some((Instruction::Cmp, AddressingMode::ZeropageXIndexed)),       // 0xD5
    Some((Instruction::Dec, AddressingMode::ZeropageXIndexed)),       // 0xD6
    None,                                                             // 0xD7
    Some((Instruction::Cld, AddressingMode::Implied)),                // 0xD8
    Some((Instruction::Cmp, AddressingMode::AbsoluteYIndexed)),       // 0xD9
    None,                                                             // 0xDA
    None,                                                             // 0xDB
    None,                                                             // 0xDC
    Some((Instruction::Cmp, AddressingMode::AbsoluteXIndexed)),       // 0xDD
    Some((Instruction::Dec, AddressingMode::AbsoluteXIndexed)),       // 0xDE
    None,                                                             // 0xDF
    Some((Instruction::Cpx, AddressingMode::Immediate)),              // 0xE0
    Some((Instruction::Sbc, AddressingMode::XIndexedIndirect)),       // 0xE1
    None,                                                             // 0xE2
    None,                                                             // 0xE3
    Some((Instruction::Cpx, AddressingMode::Zeropage)),               // 0xE4
    Some((Instruction::Sbc, AddressingMode::Zeropage)),               // 0xE5
    Some((Instruction::Inc, AddressingMode::Zeropage)),               // 0xE6
    None,                                                             // 0xE7
    Some((Instruction::Inx, AddressingMode::Implied)),                // 0xE8
    Some((Instruction::Sbc, AddressingMode::Immediate)),              // 0xE9
    Some((Instruction::Nop, AddressingMode::Implied)),                // 0xEA
    None,                                                             // 0xEB
    Some((Instruction::Cpx, AddressingMode::Absolute)),               // 0xEC
    Some((Instruction::Sbc, AddressingMode::Absolute)),               // 0xED
    Some((Instruction::Inc, AddressingMode::Absolute)),               // 0xEE
    None,                                                             // 0xEF
    Some((Instruction::Beq, AddressingMode::Relative)),               // 0xF0
    Some((Instruction::Sbc, AddressingMode::IndirectYIndexed)),       // 0xF1
    None,                                                             // 0xF2
    None,                                                             // 0xF3
    None,                                                             // 0xF4
    Some((Instruction::Sbc, AddressingMode::ZeropageXIndexed)),       // 0xF5
    Some((Instruction::Inc, AddressingMode::ZeropageXIndexed)),       // 0xF6
    None,                                                             // 0xF7
    Some((Instruction::Sed, AddressingMode::Implied)),                // 0xF8
    Some((Instruction::Sbc, AddressingMode::AbsoluteYIndexed)),       // 0xF9
    None,                                                             // 0xFA
    None,                                                             // 0xFB
    None,                                                             // 0xFC
    Some((Instruction::Sbc, AddressingMode::AbsoluteXIndexed)),       // 0xFD
    Some((Instruction::Inc, AddressingMode::AbsoluteXIndexed)),       // 0xFE
    None,                                                             // 0xFF
];
