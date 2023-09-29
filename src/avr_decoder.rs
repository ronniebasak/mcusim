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

fn get_register_bit_number_from_opcode(opcode: u16) -> (u8, u8) {
    let rd = ((opcode & 0x01F0) >> 4) as u8;
    let b = (opcode & 0x0007) as u8;
    (rd, b)
}


fn get_constant7_bit_number_from_opcode(opcode: u16) -> (u8, u8) {
    let k = ((opcode & 0x03F8) >> 3) as u8;
    let s = (opcode & 0x7) as u8;
    (k, s)
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

    match second_nibble_2msb {
        0b00 => {
            let (rd, rr) = get_rd_rr_from_opcode(opcode);
            Some(AvrInstructions::CPSE(TwoRegisters { Rr: rr, Rd: rd }))
        }
        0b01 => {
            let (rd, rr) = get_rd_rr_from_opcode(opcode);
            Some(AvrInstructions::CP(TwoRegisters { Rr: rr, Rd: rd }))
        }
        0b10 => {
            let (rd, rr) = get_rd_rr_from_opcode(opcode);
            Some(AvrInstructions::SUB(TwoRegisters { Rr: rr, Rd: rd }))
        }
        0b11 => {
            let (rd, rr) = get_rd_rr_pair_from_opcode(opcode);

            if rd == rr {
                Some(AvrInstructions::ROL(OneRegister { Rd: rd }));
            }
            Some(AvrInstructions::ADC(TwoRegisters { Rd: rd, Rr: rr }))
        }
        _ => None,
    }
}

fn decode_two_series_inst(opcode: u16) -> Option<AvrInstructions> {
    let (second_nibble, rest, second_nibble_2msb) = extract_opcode_fields(opcode);


    match second_nibble_2msb {
        0b00 => {
            let (rd, rr) = get_rd_rr_from_opcode(opcode);

            if rd == rr {
                Some(AvrInstructions::TST(OneRegister { Rd: rd }));
            }
            Some(AvrInstructions::AND(TwoRegisters { Rr: rr, Rd: rd }))
        }
        0b01 => {
            let (rd, rr) = get_rd_rr_from_opcode(opcode);

            if rd == rr {
                Some(AvrInstructions::CLR(OneRegister { Rd: rd }));
            }
            Some(AvrInstructions::EOR(TwoRegisters { Rr: rr, Rd: rd }))
        }
        0b10 => {
            let (rd, rr) = get_rd_rr_from_opcode(opcode);
            Some(AvrInstructions::OR(TwoRegisters { Rr: rr, Rd: rd }))
        }
        0b11 => {
            let (rd, rr) = get_rd_rr_pair_from_opcode(opcode);
            Some(AvrInstructions::MOV(TwoRegisters { Rr: rd, Rd: rr }))
        }
        _ => None,
    }}

fn decode_three_series_inst(opcode: u16) -> Option<AvrInstructions> {
    let rd = 0x10 | ((opcode & 0x00F0) >> 4) as u8;
    let k = ((opcode & 0x0F00) >> 4) as u8 | (opcode & 0x000F) as u8;
    Some(AvrInstructions::CPI(OneRegisterConstantValue { Rd: rd, K: k }))
}

fn decode_four_series_inst(opcode: u16) -> Option<AvrInstructions> {
    let rd = 0x10 | ((opcode & 0x00F0) >> 4) as u8;
    let k = ((opcode & 0x0F00) >> 4) as u8 | (opcode & 0x000F) as u8;
    Some(AvrInstructions::SBCI(OneRegisterConstantValue { Rd: rd, K: k }))
}

fn decode_five_series_inst(opcode: u16) -> Option<AvrInstructions> {
    let rd = 0x10 | ((opcode & 0x00F0) >> 4) as u8;
    let k = ((opcode & 0x0F00) >> 4) as u8 | (opcode & 0x000F) as u8;
    Some(AvrInstructions::SUBI(OneRegisterConstantValue { Rd: rd, K: k }))
}

fn decode_six_series_inst(opcode: u16) -> Option<AvrInstructions> {
    let rd = 0x10 | ((opcode & 0x00F0) >> 4) as u8;
    let k = ((opcode & 0x0F00) >> 4) as u8 | (opcode & 0x000F) as u8;
    // ORI and SBR are the same instruction
    Some(AvrInstructions::ORI(OneRegisterConstantValue { Rd: rd, K: k }))
}

fn decode_seven_series_inst(opcode: u16) -> Option<AvrInstructions> {
    let rd = 0x10 | ((opcode & 0x00F0) >> 4) as u8;
    let k = ((opcode & 0x0F00) >> 4) as u8 | (opcode & 0x000F) as u8;
    // CBR is silently converted to ANDI with the mask inverted by the assembler
    Some(AvrInstructions::ANDI(OneRegisterConstantValue { Rd: rd, K: k }))
}

fn decode_indirect_displacement_series_inst(opcode: u16) -> Option<AvrInstructions> {
    // ToDo
    let q = ((opcode & 0x2000 >> 8) | (opcode & 0x0C00 >> 7) | (opcode & 0x0007)) as u8;
    let r = ((opcode & 0x01F0) >> 4) as u8;

    match opcode & 0x0200 {
        0x0200 => {
            Some(AvrInstructions::STD(OneRegisterDisplaceMent { Rd: r, q }))
        }
        _ => {
            Some(AvrInstructions::LDD(OneRegisterDisplaceMent { Rd: r, q }))
        }
    }
}


fn decode_nine_series_inst(opcode: u16) -> Option<AvrInstructions> {
    let (second_nibble, rest, second_nibble_2msb) = extract_opcode_fields(opcode);
    // TODO: very long
    None
}


fn decode_eleven_series_inst(opcode: u16) -> Option<AvrInstructions> {
    let addr = ((opcode & 0x0600) >> 5 | (opcode & 0x000F)) as u8;
    let register = ((opcode & 0x01F0) >> 4) as u8;

    match opcode {
        opcode if (opcode & 0x0800) == 0x0800 => {
            Some(AvrInstructions::OUT(OneRegisterConstantAddress { Rd: register, A: addr }))
        },
        _ => {
            Some(AvrInstructions::IN(OneRegisterConstantAddress { Rd: register, A: addr }))
        }
    }
}

fn decode_twelve_series_inst(opcode: u16) -> Option<AvrInstructions> {
    Some(AvrInstructions::RJMP(Constant12Bit { K: opcode & 0x0FFF }))
}

fn decode_thirteen_series_inst(opcode: u16) -> Option<AvrInstructions> {
    Some(AvrInstructions::RCALL(Constant12Bit { K: opcode & 0x0FFF }))
}

fn decode_fourteen_series_inst(opcode: u16) -> Option<AvrInstructions> {
    let rd = 0x10 | ((opcode & 0x00F0) >> 4) as u8;
    let k = ((opcode & 0x0F00) >> 4) as u8 | (opcode & 0x000F) as u8;
    Some(AvrInstructions::LDI(OneRegisterConstantValue { Rd: rd, K: k }))
}

fn decode_fifteen_series_inst(opcode: u16) -> Option<AvrInstructions> {
    match opcode {
        opcode if (opcode & 0x0E00) == 0x0C00 => {
            let (rd, b) = get_register_bit_number_from_opcode(opcode);
            Some(AvrInstructions::SBRC(RegisterIdAndBitNumber { R: rd, b }))
        }
        opcode if (opcode & 0x0E00) == 0x0E00 => {
            let (rd, b) = get_register_bit_number_from_opcode(opcode);
            Some(AvrInstructions::SBRS(RegisterIdAndBitNumber { R: rd, b }))
        }
        opcode if (opcode & 0x0E00) == 0x0A00 => {
            let (rd, b) = get_register_bit_number_from_opcode(opcode);
            Some(AvrInstructions::BST(RegisterIdAndBitNumber { R: rd, b }))
        }
        opcode if (opcode & 0x0E00) == 0x0800 => {
            let (rd, b) = get_register_bit_number_from_opcode(opcode);
            Some(AvrInstructions::BLD(RegisterIdAndBitNumber { R: rd, b }))
        }
        opcode if (opcode & 0x0C00) == 0x0000 => {
            let (k, b) = get_constant7_bit_number_from_opcode(opcode);
            Some(AvrInstructions::BRBS(ConstantAndBitNumber { K: k, b}))
        }
        opcode if (opcode & 0x0C00) == 0x0400 => {
            let (k, b) = get_constant7_bit_number_from_opcode(opcode);
            Some(AvrInstructions::BRBC(ConstantAndBitNumber { K: k, b}))
        }
        _ => None
    }
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
        opcode if (opcode & 0xF000) == 0x8000 => decode_indirect_displacement_series_inst(opcode),
        opcode if (opcode & 0xF000) == 0x9000 => decode_nine_series_inst(opcode),
        opcode if (opcode & 0xF000) == 0xA000 => decode_indirect_displacement_series_inst(opcode),
        opcode if (opcode & 0xF000) == 0xB000 => decode_eleven_series_inst(opcode),
        opcode if (opcode & 0xF000) == 0xC000 => decode_twelve_series_inst(opcode),
        opcode if (opcode & 0xF000) == 0xD000 => decode_thirteen_series_inst(opcode),
        opcode if (opcode & 0xF000) == 0xE000 => decode_fourteen_series_inst(opcode),
        opcode if (opcode & 0xF000) == 0xF000 => decode_fifteen_series_inst(opcode),
        _ => None,
    }
}