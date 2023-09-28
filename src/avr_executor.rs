use crate::cpu::{AvrCpu};
use crate::avr_instructions::{AvrInstructions};


pub fn execute(avr_cpu: &mut AvrCpu, instruction: AvrInstructions) {
    match instruction {
        AvrInstructions::NOP => {
            println!("NOP");
        },
        AvrInstructions::LSL(operand) => {
            println!("LSL {:?}", operand);
        },
        AvrInstructions::ADD(operand) => {
            println!("ADD {:?}", operand);
        },
        AvrInstructions::LDI(operand) => {
            println!("LDI {:?}", operand);
        },
        _ => {
            println!("Unknown instruction");
        }
    }
}