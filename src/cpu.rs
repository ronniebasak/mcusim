// // emulates a CPU clock with ticks
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use std::time;

use crate::constants;


pub fn cpuclock(tx: Sender<i32>, cpu_delay_secs: f64) -> thread::JoinHandle<()> {
    let th = thread::spawn(move || {
        println!("Clock spawned");

        let clk = 1;
        loop {
            thread::sleep(time::Duration::from_secs_f64(cpu_delay_secs));
            tx.send(clk).unwrap();
        }
    });

    return th;
}



pub fn cpu(rx: Receiver<i32>, ram: &mut [u8; constants::RAMSIZE]) -> thread::JoinHandle<()> {

    let registers:[u8; 32] = [0; 32];
    let sreg: u8 = 0; // status register

    let th = thread::spawn(move || {
        println!("Thread Spawned");

        loop {
            let value = rx.recv().unwrap();

            println!("CLK {}", value);
        }
    });

    return th;
}