use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::process::exit;
use clap::Parser;
use std::string::String;

// int: i32; float: f32; long: i32
struct ValueStruct {
    struct_type: i32,
    val: f32,
    timestamp: i32,
}

impl ValueStruct {
    pub fn new(struct_type: i32, val: f32, timestamp: i32) -> CData {
        let value = ValueStruct { struct_type, val, timestamp };
        let data = CData::Value(value);
        data
    }
    pub fn print(data_number: usize, value: &ValueStruct) {
        println!("Data # {} -> type: {}, val: {}, timestamp: {}", data_number, value.struct_type, value.val, value.timestamp);
    }
}

struct MValueStruct {
    struct_type: i32,
    val: Vec<f32>,
    timestamp: i32,
}

impl MValueStruct {
    pub fn new(struct_type: i32, vals: Vec<f32>, timestamp: i32) -> CData {
        let mvalue = MValueStruct { struct_type, val: vals, timestamp };
        let data = CData::Mvalue(mvalue);
        data
    }
    pub fn print(data_number: usize, mvalue: &MValueStruct) {
        println!("Data # {} -> type: {}, val_vec: {:?}, timestamp: {}", data_number, mvalue.struct_type, mvalue.val, mvalue.timestamp);
    }
}

struct MessageStruct {
    struct_type: i32,
    message: String,
}

impl MessageStruct {
    pub fn new(struct_type: i32, message: String) -> CData {
        let mes = MessageStruct { struct_type, message };
        let data = CData::Message(mes);
        data
    }
    pub fn print(data_number: usize, message: &MessageStruct) {
        println!("Data # {} -> type: {}, Message: {}", data_number, message.struct_type, message.message);
    }
}

enum CData {
    Value(ValueStruct),
    Mvalue(MValueStruct),
    Message(MessageStruct),
}

impl CData {
    fn from_file(file: &mut File, datas: &mut Vec<CData>) {
        let mut tmp_type: i32;

        let mut i32_buffer = [0u8; 4];         // buffer for i32
        let mut char_buffer = [0u8; 1];        // buffer for char
        for i in 0..100 {
            // leggo type
            file.read(&mut i32_buffer).expect("error in reading");
            tmp_type = i32::from_le_bytes(i32_buffer);
            // match type
            match tmp_type {
                // in file there are 48 bytes save for one ExportData union struct
                // in type 1 we read 12 bytes so must skip 36 bytes
                // in type 2 we read 48 bytes so no skip
                // in type 3 we read 25 bytes so must skip 23 bytes
                1 => {
                    // read
                    file.seek(SeekFrom::Current(4)).expect("skip type Value type 1");
                    file.read(&mut i32_buffer).expect("error in reading");
                    let val = f32::from_le_bytes(i32_buffer);
                    file.read(&mut i32_buffer).expect("error in reading");
                    let timestamp = i32::from_le_bytes(i32_buffer);
                    // create correct struct
                    datas.push(ValueStruct::new(tmp_type, val, timestamp));

                    file.seek(SeekFrom::Current(36)).expect("skip type 1");
                }
                2 => {
                    // read
                    file.seek(SeekFrom::Current(4)).expect("skip type Value type 2");
                    let mut vals: Vec<f32> = Vec::with_capacity(10);
                    for _j in 0..10 {
                        file.read(&mut i32_buffer).expect("error in reading");
                        let val = f32::from_le_bytes(i32_buffer);
                        vals.push(val);
                    }
                    file.read(&mut i32_buffer).expect("error in reading");
                    let timestamp = i32::from_le_bytes(i32_buffer);
                    // create correct struct
                    datas.push(MValueStruct::new(tmp_type, vals, timestamp));
                }
                3 => {
                    file.seek(SeekFrom::Current(4)).expect("skip type Value type 3");
                    // read string
                    let mut msg_in_bytes: Vec<char> = Vec::with_capacity(21);
                    for _j in 0..21 {
                        file.read(&mut char_buffer).expect("error in reading");
                        if char_buffer[0] == 0 {
                            file.seek(SeekFrom::Current(21 - _j - 1)).expect("skip type 3");
                            break;
                        }
                        let val_char = char::from(char_buffer[0]);
                        msg_in_bytes.push(val_char);
                    }
                    let message = msg_in_bytes.iter().collect();
                    // create correct struct
                    datas.push(MessageStruct::new(tmp_type, message));

                    file.seek(SeekFrom::Current(23)).expect("skip type 3");
                }
                _ => {
                    println!("Error in reading type {}", i);
                }
            }
        }
    }
}

#[derive(Parser)]
#[command(name = "demo")]
struct Args {
    #[arg(short, long)]
    path: String,
}

fn main() {
    let args = Args::parse();
    let open = File::open(args.path);
    let mut file;
    if open.is_ok() {
        file = open.unwrap();
    } else {
        println!("Error in opening file");
        exit(1);
    }

    let mut datas: Vec<CData> = Vec::new();
    CData::from_file(&mut file, &mut datas);
    for (i, data) in datas.iter().enumerate() {
        match data {
            CData::Value(x) => {
                ValueStruct::print(i + 1, x);
            }
            CData::Mvalue(x) => {
                MValueStruct::print(i + 1, x);
            }
            CData::Message(x) => {
                MessageStruct::print(i + 1, x);
            }
        }
        println!("------------------------------------------------------------------");
    }
}
