// use std::sync::mpsc::{Receiver, Sender};
// use std::thread;
// use std::time;

use crate::avr_decoder;
use crate::constants;
use crate::avr_instructions::AvrInstructions;
use crate::avr_executor;



// add a struct representing the AVR CPU
#[derive(Debug, Clone)]
pub struct AvrCpu {
    pub ram: [u8; constants::RAMSIZE],
    pub pc: u16,
    pub sp: u8,
    pub sreg: u8,
    pub registers: [u8; 32],
    pub flash: [u8; constants::FLASHSIZE],


    decoded_opcode: u16,
    decoded_instruction: AvrInstructions
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
            decoded_opcode: 0,
            decoded_instruction: AvrInstructions::NOP
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

    pub fn get_decoded_instruction(&self) -> AvrInstructions {
        return self.decoded_instruction;
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


    pub fn fetch(&mut self) {
        // read 2 bytes into a u16 in little endian
        let low_byte = self.flash[(self.pc * 2) as usize] as u16;
        let high_byte = self.flash[(self.pc * 2 + 1) as usize] as u16;
        let instruction: u16 = low_byte | (high_byte << 8);

        self.decoded_opcode = instruction;
    }

    pub fn decode(&mut self) {
        // decode the avr instructions
        self.decoded_instruction = avr_decoder::avr_decoder(self.decoded_opcode).unwrap();
    }

    pub fn execute(&mut self) {
        // noop
        return avr_executor::execute(self); 
    }
}



pub fn run(data: Vec<u8>) {
    println!("CPU RUN START");

    let mut avr_cpu = AvrCpu::new();

    if data.len() >= constants::FLASHSIZE {
        avr_cpu.flash = data[0..constants::FLASHSIZE].try_into().unwrap();
    } else {
        for (i, &value) in data.iter().enumerate() {
            avr_cpu.flash[i] = value;
        }
    }

    loop {
        if (avr_cpu.pc * 2) as usize >= data.len() {
            println!("CPU RUN END, {:?}", avr_cpu.registers);
            break;
        }

        avr_cpu.fetch();
        println!("opcode: {:x}", avr_cpu.decoded_opcode);
        avr_cpu.decode();
        println!("decoded: {:#?}", avr_cpu.decoded_instruction);
        avr_cpu.execute();

        avr_cpu.pc += 1;
    }
}
