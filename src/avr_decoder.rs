use crate::avrinstructions::{*};


fn decode_zero_series_inst(opcode: u16) -> Option<AvrInstructions> {
    let second_nibble = ((opcode & 0x0F00) >> 8) as u8;
    let rest = (opcode & 0x00FF) as u8;

    let second_nibble_2msb = (second_nibble & 0b1100) >> 2;

    if second_nibble_2msb != 0b11 {
        return None;
    }

    let rr = (((second_nibble & 0x02) << 3) | rest & 0x0F) as u8;
    let rd = ((opcode & 0x01F0) >> 4) as u8;

    if rd == rr {
        Some(AvrInstructions::LSL(OneRegister { Rd: rd }))
    } else {
        Some(AvrInstructions::ADD(TwoRegisters { Rr: rr, Rd: rd }))
    }
}

fn decode_e_series_inst(opcode: u16) -> Option<AvrInstructions> {
    let rd = 0x10 | ((opcode & 0x00F0) >> 4) as u8;
    let k = ((opcode & 0x0F00) >> 4) as u8 | (opcode & 0x000F) as u8;

    Some(AvrInstructions::LDI(OneRegisterConstantValue { Rd: rd, K: k }))
}

pub fn avr_decoder(opcode: u16) -> Option<AvrInstructions> {
    match opcode {
        0 => Some(AvrInstructions::NOP),
        opcode if (opcode & 0xF000) == 0 => decode_zero_series_inst(opcode),
        opcode if (opcode & 0xF000) == 0xE000 => decode_e_series_inst(opcode),
        _ => None,
    }
}