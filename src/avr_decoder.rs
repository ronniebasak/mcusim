use crate::avrinstructions::{AvrInstructionSignals, AvrInstructions};

#[derive(Debug, Clone)]
pub struct InstructionSet {
    instruction: AvrInstructions,
    signal: AvrInstructionSignals,
}

fn decode_zero_series_inst(opcode: u16) -> Option<InstructionSet> {
    let second_nibble = ((opcode & 0x0F00) >> 8) as u8;
    let rest = (opcode & 0x00FF) as u8;

    let second_nibble_2msb = (second_nibble & 0b1100) >> 2;

    if second_nibble_2msb == 0b11 {
        // 0b0000 11xx xxxx xxxx, ADD or LSL
        // parse the 2nd LSB as Rr MSB and LSB as Rd MSB

        let rr = (((second_nibble & 0b0010) << 3) | rest & 0x0F) as u8;
        let rd = (opcode & 0b0000_0001_1111_0000 >> 4) as u8;

        if rd == rr {
            // LSL
            return Some(InstructionSet {
                instruction: AvrInstructions::LSL,
                signal: AvrInstructionSignals {
                    rd, ..Default::default()
                },
            });
        } else {
            // ADD
            return Some(InstructionSet {
                instruction: AvrInstructions::ADD,
                signal: AvrInstructionSignals {
                    rd, rr, ..Default::default()
                },
            });
        }
    }

    return None;
}

fn decode_e_series_inst(opcode: u16) -> Option<InstructionSet>{
    let rd = 0x10 | (opcode & 0x00F0 >> 4) as u8;
    let k1 = (opcode & 0x0F00 >> 8) as u8;
    let k2 = (opcode & 0x000F) as u8;
    let k = (k1 << 4) | k2;

    return Some(InstructionSet {
        instruction: AvrInstructions::LDI,
        signal: AvrInstructionSignals {
            rd, k, ..Default::default()
        },
    });
}

pub fn avr_decoder(opcode: u16) -> Option<InstructionSet>{
    // read the first nibble
    let first_nibble = ((opcode & 0xF000) >> 12) as u8;
    // read the second nibble
    let second_nibble = ((opcode & 0x0F00) >> 8) as u8;
    // read the third nibbl
    let rest = opcode & 0x00FF;

    if opcode == 0 {
        return Some(InstructionSet {
            instruction: AvrInstructions::NOP,
            signal: AvrInstructionSignals {
                ..Default::default()
            },
        });
    } else if first_nibble == 0x0 {
        return decode_zero_series_inst(opcode);
    } else if first_nibble == 0b1110 {
        return decode_e_series_inst(opcode);
    }

    return None;
}
