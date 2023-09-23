use crate::constants::*;
use std::fs::File;
use std::io::Read;
use std::{default, fs};

const STATE_IGNORING: u8 = 0;
const STATE_PARSE_BYTECOUNT: u8 = 1;
const STATE_PARSE_ADDR: u8 = 2;
const STATE_PROCESS_ADDR: u8 = 3;
const STATE_PARSE_RECORD_TYPE: u8 = 4;
const STATE_PARSE_DATA: u8 = 5;
const STATE_VERIFY_CKSUM: u8 = 6;
const STATE_FINISH: u8 = 7;

const RECORD_DATA: u8 = 0;
const RECORD_EOF: u8 = 1;
const RECORD_ESA: u8 = 2; // Extended Segment Address
const RECORD_SSA: u8 = 3; // Start Segment Address
const RECORD_ELA: u8 = 4; // Extended Linear Address
const RECORD_SLA: u8 = 5; // Start Linear Address

#[derive(Default, Debug)]
pub struct Record {
    byte_count: u8,
    address: u16,
    record_type: RecordType,
    data: u8,
}

#[derive(Debug)]
pub enum RecordType {
    DATA, // 00
    EOF,  // 01
    ESA,  // 02 Extended Segment Address
    SSA,  // 03 Start Segment Address
    ELA,  // 04 Extended Linear Address
    SLA,  // 05 Start Linear Address
}

impl Default for RecordType {
    fn default() -> Self {
        RecordType::DATA
    }
}

fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

fn parse_single_record(_bdata: &Vec<u8>, _bdataptr: &mut usize) -> Record {
    let mut rec = Record::default();

    let mut state: u8 = STATE_IGNORING;

    // ignore until start byte is not found
    while state == STATE_IGNORING && _bdata[*_bdataptr] != INTEL_HEX_DELIM {
        println!("Found Start byte");
        state += 1; // inc the state machine
        *_bdataptr += 1;
    }

    // find byte count
    if state == STATE_PARSE_BYTECOUNT {
        let byte = _bdata[*_bdataptr];
        rec.byte_count = byte;
        state += 1; // inc the state machine
        *_bdataptr += 1;
    }

    if state == STATE_PARSE_ADDR {
        let byte = _bdata[*_bdataptr];
        let nextbyte = _bdata[*_bdataptr + 1];
        let addr: u16 = (byte as u16) << 8 | (nextbyte as u16);
        rec.address = addr;
        state += 1;
        *_bdataptr += 2
    }

    if state == STATE_PROCESS_ADDR {
        state += 1; // noop
                    // *_bdataptr += 1
    }

    if state == STATE_PARSE_RECORD_TYPE {
        let byte = _bdata[*_bdataptr];
        match byte {
            RECORD_DATA => rec.record_type = RecordType::DATA,
            RECORD_EOF => rec.record_type = RecordType::EOF,
            RECORD_ESA => rec.record_type = RecordType::ESA,
            RECORD_SSA => rec.record_type = RecordType::SSA,
            RECORD_ELA => rec.record_type = RecordType::ELA,
            RECORD_SLA => rec.record_type = RecordType::SLA,
            _ => panic!("Not a valid record"),
        }
    }

    rec
}

pub fn parse(file_name: String) -> Vec<Record> {
    let ret = vec![Record {
        byte_count: 0,
        address: 0,
        record_type: 0,
        data: 0,
    }];

    let bdata = get_file_as_byte_vec(&file_name);
    let recvec = Vec::<Record>::new();
    let mut bdataptr: usize = 0;

    while bdataptr < bdata.len() {
        let _ = parse_single_record(&bdata, &mut bdataptr);
    }

    ret
}
