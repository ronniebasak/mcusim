use crate::cpu::{AvrCpu};
use crate::avr_instructions::{AvrInstructions};


pub fn execute(avr_cpu: &mut AvrCpu) {
    let instruction = avr_cpu.get_decoded_instruction();
    match instruction {
        AvrInstructions::NOP => {
            println!("NOP");
        },
        AvrInstructions::LSL(operand) => {
            println!("LSL {:?}", operand);
            avr_cpu.registers[operand.Rd as usize] = avr_cpu.registers[operand.Rd as usize] << 1;
        },
        AvrInstructions::ADD(operand) => {
            println!("ADD {:?}", operand);
            avr_cpu.registers[operand.Rd as usize] = avr_cpu.registers[operand.Rd as usize] + avr_cpu.registers[operand.Rr as usize];
        },
        AvrInstructions::LDI(operand) => {
            println!("LDI {:?}", operand);
            avr_cpu.registers[operand.Rd as usize] = operand.K;
        },
        _ => {
            println!("Unknown instruction");
        }
    }
}