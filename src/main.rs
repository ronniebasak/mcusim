pub mod cpu;
pub mod constants;
pub mod IntelHexParse;

use std::sync::mpsc::channel;
use std::{thread, time};

use IntelHexParse::parse;



fn main() {
    // let cpu_freq: u32 = 2;
    // let cpu_delay_secs = (1.0 as f64) / (cpu_freq as f64);

    // let mut ram: [u8; constants::RAMSIZE] = [0 as u8; constants::RAMSIZE];

    // // setting up the 32 bit wide BUS
    // let (tx, rx) = channel::<i32>();


    // // clock and cpu runs on seperate threads 
    // let cpuclock = cpu::cpuclock(tx, cpu_delay_secs);
    // let cpu = cpu::cpu(rx, &mut ram);

    // cpuclock.join().expect("The sender thread as panicked");
    // cpu.join().expect("The receiver thread has panicked");


    let file_name = "/home/sohan/Downloads/temp/out/add.hex".to_owned();
    parse(file_name);
}
