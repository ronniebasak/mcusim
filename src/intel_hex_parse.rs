use std::fs::File;
use std::io::{BufRead, BufReader};
use hex;
use std::num::Wrapping;

// Record Type enum
#[derive(Debug)]
enum RecordType {
    Data,
    EndOfFile,
    ExtendedSegmentAddress,
    StartSegmentAddress,
    ExtendedLinearAddress,
    StartLinearAddress,
}

impl Default for RecordType {
    fn default() -> Self {
        RecordType::Data
    }
}

// add a struct to hold each line of the hex file
#[derive(Debug, Default)]
struct HexLine {
    byte_count: u8,
    address: u16,
    record_type: RecordType,
    data: Vec<u8>,
    checksum: u8,
}

fn parse_hex_line(line: &str) -> HexLine {
    let bytes = hex::decode(&line[1..]).unwrap();
    let byte_count = bytes[0];
    let address = ((bytes[1] as u16) << 8) | (bytes[2] as u16);
    let record_type_byte = bytes[3];
    let record_type: RecordType = match record_type_byte {
        0x00 => RecordType::Data,
        0x01 => RecordType::EndOfFile,
        0x02 => RecordType::ExtendedSegmentAddress,
        0x03 => RecordType::StartSegmentAddress,
        0x04 => RecordType::ExtendedLinearAddress,
        0x05 => RecordType::StartLinearAddress,
        _ => panic!("Invalid record type"),
    };

    let data = bytes[4..byte_count as usize + 4].to_vec();
    let checksum = bytes[byte_count as usize + 4];

    // validate intel hex checksum
    let sum: Wrapping<u8> = bytes.iter().map(|&b| Wrapping(b)).sum();
    if sum != Wrapping(0) {
        panic!("Invalid checksum");
    }

    HexLine {
        byte_count,
        address,
        record_type,
        data,
        checksum,
    }
}


pub fn parse(filename: String) -> Vec<HexLine> {
    let file = File::open(filename).unwrap();
    let reader: BufReader<File> = BufReader::new(file);
    let mut retval: Vec<HexLine> = Vec::new(); 

    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with(':') {
            let hex_line = parse_hex_line(&line);

            retval.push(hex_line);
            // Do something with the parsed data
            println!("Address: {:04X}, Data: {:?}", hex_line.address, hex_line.data);
        }
    }

    return retval;
}