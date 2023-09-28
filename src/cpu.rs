// use std::sync::mpsc::{Receiver, Sender};
// use std::thread;
// use std::time;

use crate::avr_decoder;
use crate::constants;
use crate::avr_instructions::AvrInstructions;
use create::avr_executor;

// add a struct representing the AVR CPU
pub struct AvrCpu {
    pub ram: [u8; constants::RAMSIZE],
    pub pc: u16,
    pub sp: u8,
    pub sreg: u8,
    pub registers: [u8; 32],
    pub flash: [u8; constants::FLASHSIZE],
}

impl AvrCpu {
    pub fn new() -> AvrCpu {
        AvrCpu {
            ram: [0; constants::RAMSIZE],
            pc: 0,
            sp: 0,
            sreg: 0,
            registers: [0; 32],
            flash: [0; constants::FLASHSIZE],
        }
    }

    pub fn reset(&mut self) {
        self.ram = [0; constants::RAMSIZE];
        self.pc = 0;
        self.sp = 0;
        self.sreg = 0;
        self.registers = [0; 32];
        self.flash = [0; constants::FLASHSIZE];
    }

    pub fn set_flash(&mut self, data: Vec<u8>) {
        if data.len() >= constants::FLASHSIZE {
            self.flash = data[0..constants::FLASHSIZE].try_into().unwrap();
        } else {
            for (i, &value) in data.iter().enumerate() {
                self.flash[i] = value;
            }
        }
    }

    pub fn set_ram(&mut self, data: Vec<u8>) {
        if data.len() >= constants::RAMSIZE {
            self.ram = data[0..constants::RAMSIZE].try_into().unwrap();
        } else {
            for (i, &value) in data.iter().enumerate() {
                self.ram[i] = value;
            }
        }
    }


    pub fn fetch(&mut self) -> u16 {
        // read 2 bytes into a u16 in little endian
        let low_byte = self.flash[(self.pc * 2) as usize] as u16;
        let high_byte = self.flash[(self.pc * 2 + 1) as usize] as u16;
        let instruction: u16 = low_byte | (high_byte << 8);
        return instruction;
    }

    pub fn decode(&mut self, opcode: u16) -> AvrInstructions {
        // decode the avr instructions
        return avr_decoder::avr_decoder(opcode).unwrap();
    }

    pub fn execute(&mut self, instruction: AvrInstructions) {
        // noop
        return avr_executor::execute(instruction); 
    }
}


+
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

    if data.len() >= constants::FLASHSIZE {
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
