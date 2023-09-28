pub mod cpu;
pub mod constants;
pub mod intel_hex_parse;
pub mod avr_decoder;
pub mod avr_instructions;
pub mod avr_executor;


use intel_hex_parse::parse;
use intel_hex_parse::RecordType;

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


    let file_name: String = "/home/sohan/Downloads/temp/out/add.hex".to_owned();
    let hex_contents = parse(file_name);

    let mut data: Vec<u8> = Vec::new();
    for item in hex_contents.iter() {
        println!("{:?}", item);
        if item.record_type == RecordType::Data {
            println!("MDATA: {:?}", item.data);
            data.append(item.data.clone().as_mut());
        }
    }

    cpu::run(data);
}
