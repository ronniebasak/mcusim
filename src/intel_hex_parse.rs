use std::fs::File;
use std::io::{BufRead, BufReader};
use hex;

// add a struct to hold each line of the hex file
struct HexLine {
    byte_count: u8,
    address: u16,
    record_type: u8,
    data: Vec<u8>,
    checksum: u8,
}

fn parse_hex_line(line: &str) -> HexLine {
    let bytes = hex::decode(&line[1..]).unwrap();
    let byte_count = bytes[0];
    let address = ((bytes[1] as u16) << 8) | (bytes[2] as u16);
    let record_type = bytes[3];
    let data = bytes[4..byte_count as usize + 4].to_vec();
    let checksum = bytes[byte_count as usize + 4];

    // validate intel hex checksum
    let sum: u8 = bytes.iter().sum();
    if sum != 0 {
        println!("Invalid checksum");
        // panic!("Invalid checksum");
    }

    HexLine {
        byte_count,
        address,
        record_type,
        data,
        checksum,
    }
}


pub fn parse(filename: String) {
    let file = File::open(filename).unwrap();
    let reader: BufReader<File> = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with(':') {
            let hex_line = parse_hex_line(&line);

            // Do something with the parsed data
            println!("Address: {:04X}, Data: {:?}", hex_line.address, hex_line.data);
        }
    }

}