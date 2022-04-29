use std::u8;
use std::io;
use std::io::Read;
use std::io::BufReader;
use std::fs::File;

fn main() {
    println!("Hello, world!");
    let mut code_buffer : Vec<u8> = Vec::new();
    code_buffer.push(2);
    disassemble_8080(&mut code_buffer, 231);
    read_file_to_buffer("../invaders.h");
}
fn read_file_to_buffer(path : &str){
    let f = File::open(path).expect("no file found");
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();
    
    // Read file into vector.
    reader.read_to_end(&mut buffer).expect("Failed to read buffer");
    
    // Read.
    let mut x = 0;

    for value in buffer {
        let code = match value{
            0x01 => "LXI",
            0x02 => "STAX B",
            0x03 => "INX B",
            0x04 => "INR B",
            0x05 => "DCR B",
            0x06 => "MVI B, D8",
            0x07 => "RLC",
            0x08 => "-",
            0x09 => "DAD B",
            0x0A => "LDAX B",
            0x0B => "DCX B",
            0x0C => "INR C",
            0x0D => "DCR C",
            0x0E => "MVI C,D8",
            0x0F => "RRC",
            0x10 => "-",
            0x11 => "LXI D,D16",
            0x12 => "STAX D",
            0x13 => "INX D",
            0x14 => "INR D",
            0x15 => "DCR D",
            0x16 => "MVI D,D8",
            0x17 => "RAL",
            0x18 => "-",
            number => "Other",
        };
        println!("BYTE: {}", code);
    }

}
fn disassemble_8080(code_buffer: &mut [u8], pc: usize)  {
    // let mut code : u8 = code_buffer[pc];
    println!("Program Counter: {:04x}", pc);
}