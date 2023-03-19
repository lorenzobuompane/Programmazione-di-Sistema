use std::fs::File;
use std::io::Read;
use std::mem::{ManuallyDrop};
use std::process::exit;
use clap::Parser;
use std::string::String;
use std::str;

// int: i32; float: f32; long: i32
struct ValueStruct {
    struct_type: i32,
    val: i32,
    timestamp: i32
}

struct MValueStruct {
    struct_type: i32,
    val: Vec<i32>,
    timestamp: i32
}

struct MessageStruct {
    struct_type: i32,
    message: String
}

union DataType {
    value: std::mem::ManuallyDrop<ValueStruct>,
    mvalue: std::mem::ManuallyDrop<MValueStruct>,
    message: std::mem::ManuallyDrop<MessageStruct>
}

struct CData {
    struct_type: i32,
    data: DataType
}

impl CData {
    fn from_file(file: &mut File) -> Vec<CData> {
        let mut datas: Vec<CData> = Vec::with_capacity(100);
        let mut tmp_type: i32;

        let mut i32_buffer = [0u8; 4];         // buffer for i32
        let mut char_buffer = [0u8; 1];        // buffer for char
        for i in 0 .. 100 {
            // leggo type
            let mut bytes_read = file.read( &mut i32_buffer);
            tmp_type = i32::from_le_bytes(i32_buffer);
            // match type
            match tmp_type {
                1 => {
                    // read
                    bytes_read = file.read( &mut i32_buffer);
                    let val = i32::from_le_bytes(i32_buffer);
                    bytes_read = file.read( &mut i32_buffer);
                    let timestamp = i32::from_le_bytes(i32_buffer);
                    // create correct struct
                    let value = ValueStruct{struct_type: tmp_type, val, timestamp};
                    let mdrop = ManuallyDrop::new(value);
                    let data = CData{struct_type:tmp_type, data: DataType{value: mdrop}};
                    // push
                    datas.push(data);
                },
                2 => {
                    // read
                    let mut vals: Vec<i32> = Vec::with_capacity(10);
                    for j in 0 .. 10 {
                        bytes_read = file.read( &mut i32_buffer);
                        let val = i32::from_le_bytes(i32_buffer);
                        vals.push(val);
                    }
                    bytes_read = file.read( &mut i32_buffer);
                    let timestamp = i32::from_le_bytes(i32_buffer);
                    // create correct struct
                    let mvalue = MValueStruct{struct_type: tmp_type, val: vals, timestamp};
                    let mdrop = ManuallyDrop::new(mvalue);
                    let data = CData{struct_type:tmp_type, data: DataType{mvalue: mdrop}};
                    // push
                    datas.push(data);
                }
                3 => {
                    // read string
                    let mut msg_in_bytes:Vec<u8> = Vec::with_capacity(21);
                    for j in 0 .. 21 {
                        bytes_read = file.read( &mut char_buffer);
                        let val = u8::from_le_bytes(char_buffer);
                        msg_in_bytes.push(val);
                    }
                    let message= str::from_utf8(&msg_in_bytes).unwrap().to_string();
                    // create correct struct
                    let mes = MessageStruct{struct_type: tmp_type, message};
                    let mdrop = ManuallyDrop::new(mes);
                    let data = CData{struct_type:tmp_type, data: DataType{message: mdrop}};
                    // push
                    datas.push(data);
                }
                _ => {
                    println!("Error in reading type");
                }
            }
            }
        datas
    }
}
#[derive(Parser)]
#[command(name = "demo")]
struct Args{
    #[arg(short, long)]
    path: String
}

fn main() {
    let args = Args::parse();
    let mut open = File::open(args.path);
    let mut file;
    if open.is_ok() {
        file = open.unwrap();
    } else {
        println!("Error in opening file");
        exit(1);
    }

    let import = CData::from_file(&mut file);

        /*for i in 0..100 {
            match import[i].struct_type {
                1 => {
                    println!("Data # {} -> type: {}, val: {}, timestamp: {}", i + 1, import[i].data.value.struct_type, import[i].data.value.val, import[i].data.value.timestamp)
                },
                2 => {
                    println!("Data # {} -> type: {}, val_vec: {:?}, timestamp: {}", i + 1, import[i].data.mvalue.struct_type, import[i].data.mvalue.val, import[i].data.mvalue.timestamp)
                }
                3 => {
                    println!("Data # {} -> type: {}, val: {}, timestamp: {}", i + 1, import[i].data.value.struct_type, import[i].data.value.val, import[i].data.value.timestamp)
                }
                _ => {
                    println!("Error in printing");
                }
            }*/



    //print_datas(&import);

}
/*
fn print_datas(datas: &Vec<CData>) {
    for i in 0..100 {
        match datas[i].struct_type {
            1 => {
                println!("Data # {} -> type: {}, val: {}, timestamp: {}", i + 1, datas[i].data.value.struct_type, datas[i].data.value.val, datas[i].data.value.timestamp)
            },
            2 => {
                println!("Data # {} -> type: {}, val_vec: {:?}, timestamp: {}", i + 1, datas[i].data.mvalue.struct_type, datas[i].data.mvalue.val, datas[i].data.mvalue.timestamp)
            }
            3 => {
                println!("Data # {} -> type: {}, val: {}, timestamp: {}", i + 1, datas[i].data.value.struct_type, datas[i].data.value.val, datas[i].data.value.timestamp)
            }
            _ => {
                println!("Error in printing");
            }
        }
    }
}*/

