use crate::avr_instructions::{*};


fn get_rd_rr_from_opcode(opcode: u16) -> (u8, u8) {
    let rd = ((opcode & 0x01F0) >> 4) as u8;
    let rr = ((opcode & 0x0200) >> 5) as u8 | (opcode & 0x000F) as u8;

    (rd, rr)
}

// used in movw
fn get_rd_rr_pair_from_opcode(opcode: u16) -> (u8, u8) {
    let _rd = ((opcode & 0x00F0) >> 4) as u8;
    let _rr = (opcode & 0x000F) as u8 | (opcode & 0x000F) as u8;

    let rd = (_rd << 1);
    let rr = (_rr << 1);

    (rd, rr)
}



fn extract_opcode_fields(opcode: u16) -> (u8, u8, u8) {
    let second_nibble = ((opcode & 0x0F00) >> 8) as u8;
    let rest = (opcode & 0x00FF) as u8;
    let second_nibble_2msb = (second_nibble & 0b1100) >> 2;

    (second_nibble, rest, second_nibble_2msb)
}



fn decode_zero_series_inst(opcode: u16) -> Option<AvrInstructions> {
    let (second_nibble, rest, second_nibble_2msb) = extract_opcode_fields(opcode);


    match second_nibble_2msb {
        0b11 => {
            let (rd, rr) = get_rd_rr_from_opcode(opcode);
            if rd == rr {
                Some(AvrInstructions::LSL(OneRegister { Rd: rd }))
            } else {
                Some(AvrInstructions::ADD(TwoRegisters { Rr: rr, Rd: rd }))
            }
        }
        0b10 => {
            let (rd, rr) = get_rd_rr_from_opcode(opcode);
            Some(AvrInstructions::SBC(TwoRegisters { Rr: rr, Rd: rd }))
        }
        0b01 => {
            let (rd, rr) = get_rd_rr_from_opcode(opcode);
            Some(AvrInstructions::CPC(TwoRegisters { Rr: rr, Rd: rd }))
        }
        0b00 => {
            let (rd, rr) = get_rd_rr_pair_from_opcode(opcode);
            Some(AvrInstructions::MOVW(TwoRegistersPair { Rd_lower: rd, Rr_lower: rr }))
        }
        _ => None,
    }

}

fn decode_one_series_inst(opcode: u16) -> Option<AvrInstructions> {
    let (second_nibble, rest, second_nibble_2msb) = extract_opcode_fields(opcode);

    None
}

fn decode_two_series_inst(opcode: u16) -> Option<AvrInstructions> {
    let (second_nibble, rest, second_nibble_2msb) = extract_opcode_fields(opcode);

    None
}

fn decode_three_series_inst(opcode: u16) -> Option<AvrInstructions> {
    let (second_nibble, rest, second_nibble_2msb) = extract_opcode_fields(opcode);

    None
}

fn decode_four_series_inst(opcode: u16) -> Option<AvrInstructions> {
    let (second_nibble, rest, second_nibble_2msb) = extract_opcode_fields(opcode);

    None
}

fn decode_five_series_inst(opcode: u16) -> Option<AvrInstructions> {
    let (second_nibble, rest, second_nibble_2msb) = extract_opcode_fields(opcode);

    None
}

fn decode_six_series_inst(opcode: u16) -> Option<AvrInstructions> {
    let (second_nibble, rest, second_nibble_2msb) = extract_opcode_fields(opcode);

    None
}

fn decode_seven_series_inst(opcode: u16) -> Option<AvrInstructions> {
    let (second_nibble, rest, second_nibble_2msb) = extract_opcode_fields(opcode);

    None
}

fn decode_eight_series_inst(opcode: u16) -> Option<AvrInstructions> {
    let (second_nibble, rest, second_nibble_2msb) = extract_opcode_fields(opcode);

    None
}

fn decode_nine_series_inst(opcode: u16) -> Option<AvrInstructions> {
    let (second_nibble, rest, second_nibble_2msb) = extract_opcode_fields(opcode);

    None
}

fn decode_ten_series_inst(opcode: u16) -> Option<AvrInstructions> {
    let (second_nibble, rest, second_nibble_2msb) = extract_opcode_fields(opcode);

    None
}

fn decode_eleven_series_inst(opcode: u16) -> Option<AvrInstructions> {
    let (second_nibble, rest, second_nibble_2msb) = extract_opcode_fields(opcode);

    None
}

fn decode_twelve_series_inst(opcode: u16) -> Option<AvrInstructions> {
    let (second_nibble, rest, second_nibble_2msb) = extract_opcode_fields(opcode);

    None
}

fn decode_thirteen_series_inst(opcode: u16) -> Option<AvrInstructions> {
    let (second_nibble, rest, second_nibble_2msb) = extract_opcode_fields(opcode);

    None
}

fn decode_fourteen_series_inst(opcode: u16) -> Option<AvrInstructions> {
    let (_, _, second_nibble_2msb) = extract_opcode_fields(opcode);

    let rd = 0x10 | ((opcode & 0x00F0) >> 4) as u8;
    let k = ((opcode & 0x0F00) >> 4) as u8 | (opcode & 0x000F) as u8;

    Some(AvrInstructions::LDI(OneRegisterConstantValue { Rd: rd, K: k }))
}

fn decode_fifteen_series_inst(opcode: u16) -> Option<AvrInstructions> {
    let (second_nibble, rest, second_nibble_2msb) = extract_opcode_fields(opcode);

    None
}

fn decode_sixteen_series_inst(opcode: u16) -> Option<AvrInstructions> {
    let (second_nibble, rest, second_nibble_2msb) = extract_opcode_fields(opcode);

    None
}


pub fn avr_decoder(opcode: u16) -> Option<AvrInstructions> {
    match opcode {
        0 => Some(AvrInstructions::NOP),
        opcode if (opcode & 0xF000) == 0 => decode_zero_series_inst(opcode),
        opcode if (opcode & 0xF000) == 0x1000 => decode_one_series_inst(opcode),
        opcode if (opcode & 0xF000) == 0x2000 => decode_two_series_inst(opcode),
        opcode if (opcode & 0xF000) == 0x3000 => decode_three_series_inst(opcode),
        opcode if (opcode & 0xF000) == 0x4000 => decode_four_series_inst(opcode),
        opcode if (opcode & 0xF000) == 0x5000 => decode_five_series_inst(opcode),
        opcode if (opcode & 0xF000) == 0x6000 => decode_six_series_inst(opcode),
        opcode if (opcode & 0xF000) == 0x7000 => decode_seven_series_inst(opcode),
        opcode if (opcode & 0xF000) == 0x8000 => decode_eight_series_inst(opcode),
        opcode if (opcode & 0xF000) == 0x9000 => decode_nine_series_inst(opcode),
        opcode if (opcode & 0xF000) == 0xA000 => decode_ten_series_inst(opcode),
        opcode if (opcode & 0xF000) == 0xB000 => decode_eleven_series_inst(opcode),
        opcode if (opcode & 0xF000) == 0xC000 => decode_twelve_series_inst(opcode),
        opcode if (opcode & 0xF000) == 0xD000 => decode_thirteen_series_inst(opcode),
        opcode if (opcode & 0xF000) == 0xE000 => decode_fourteen_series_inst(opcode),
        opcode if (opcode & 0xF000) == 0xF000 => decode_fifteen_series_inst(opcode),
        _ => None,
    }
}