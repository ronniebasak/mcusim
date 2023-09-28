// use std::sync::mpsc::{Receiver, Sender};
// use std::thread;
// use std::time;

use crate::avr_decoder;
use crate::constants;

// add a struct representing the AVR CPU
struct AvrCpu {
    ram: [u8; constants::RAMSIZE],
    pc: u16,
    sp: u8,
    sreg: u8,
    registers: [u8; 32],
    flash: [u8; constants::FLASHSIZE],
}

fn fetch(avr_cpu: &AvrCpu) -> u16 {
    // read 2 bytes into a u16 in little endian
    let low_byte = avr_cpu.flash[(avr_cpu.pc * 2) as usize] as u16;
    let high_byte = avr_cpu.flash[(avr_cpu.pc * 2 + 1) as usize] as u16;
    let instruction: u16 = low_byte | (high_byte << 8);
    return instruction;
}

fn decode(opcode: u16) -> avr_decoder::InstructionSet {
    // decode the avr instructions
    return avr_decoder::avr_decoder(opcode).unwrap();
}

fn execute() {
    // noop
}

pub fn run(data: Vec<u8>) {
    println!("CPU RUN START");

    let mut avr_cpu = AvrCpu {
        ram: [0; constants::RAMSIZE],
        pc: 0,
        sp: 0,
        sreg: 0,
        registers: [0; 32],
        flash: [0; constants::FLASHSIZE],
    };

    if data.len() > constants::FLASHSIZE {
        avr_cpu.flash = data[0..constants::FLASHSIZE].try_into().unwrap();
    } else {
        for (i, &value) in data.iter().enumerate() {
            avr_cpu.flash[i] = value;
        }
    }

    loop {
        if (avr_cpu.pc * 2) as usize >= data.len() {
            break;
        }

        let opcode = fetch(&avr_cpu);
        avr_cpu.pc += 1;

        println!("opcode: {:x}", opcode);

        let decoded = decode(opcode);

        println!("decoded: {:#?}", decoded);
        execute();
    }
}
