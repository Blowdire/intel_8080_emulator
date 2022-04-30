use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::LineWriter;
use std::io::Read;
use std::io::Write;
use std::u8;

fn main() {
    //read rom file
    let code_buffer = read_file_to_buffer("../invaders.g");
    //initialize program counter
    let mut pc: usize = 0;
    let buffer_length = code_buffer.len();
    let state_flags = ConditionFlags {
        zero: false,
        parity: false,
        sign: false,
        carry: false,
        aux_carry: false,
    };

    while pc < buffer_length {
        pc += disassemble_8080(&code_buffer, pc)
    }
}

fn read_file_to_buffer(path: &str) -> Vec<u8> {
    let f = File::open(path).expect("no file found");
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();

    // Read file into vector.
    reader
        .read_to_end(&mut buffer)
        .expect("Failed to read buffer");

    // Read.
    buffer
}

fn disassemble_8080(code_buffer: &Vec<u8>, pc: usize) -> usize {
    let current_code: u8 = code_buffer[pc];
    let mut num_bytes: usize = 1;
    // println!("Program Counter: {:04x}", pc);
    let code = match current_code {
        0x00 => String::from("NOP"),
        0x01 => {
            num_bytes = 3;
            format!(
                "LXI\tB, {:02x}{:02x}",
                code_buffer[pc + 2],
                code_buffer[pc + 1]
            )
        }
        0x02 => String::from("STAX\tB"),
        0x03 => String::from("INX\tB"),
        0x04 => String::from("INR\tB"),
        0x05 => String::from("DCR\tB"),
        0x06 => {
            num_bytes = 2;
            format!("MVI\tB, {:02x}", code_buffer[pc + 1])
        }
        0x07 => String::from("RLC"),
        0x08 => String::from("NOP"),
        0x09 => String::from("DAD\tB"),
        0x0A => String::from("LDAX\tB"),
        0x0B => String::from("DCX\tB"),
        0x0C => String::from("INR\tC"),
        0x0D => String::from("DCR\tC"),
        0x0E => {
            num_bytes = 2;
            format!("MVI\tC,{:02x}", code_buffer[pc + 1])
        }
        0x0F => String::from("RRC"),
        0x10 => String::from("NOP"),
        0x11 => {
            num_bytes = 2;
            format!("LXI\tD,{:02x}", code_buffer[pc + 1])
        }
        0x12 => String::from("STAX\tD"),
        0x13 => String::from("INX\tD"),
        0x14 => String::from("INR\tD"),
        0x15 => String::from("DCR\tD"),
        0x16 => {
            num_bytes = 2;
            format!("MVI\tD,{:02x}", code_buffer[pc + 1])
        }
        0x17 => String::from("RAL"),
        0x18 => String::from("NOP"),
        0x19 => String::from("DAD\tD"),
        0x1a => String::from("LDAX\tD"),
        0x1b => String::from("DCX\tD"),
        0x1c => String::from("INR\tE"),
        0x1d => String::from("DCR\tE"),
        0x1e => {
            num_bytes = 2;
            format!("MVI\tE, {:02x}", code_buffer[pc + 1])
        }
        0x1f => String::from("RAR"),
        0x20 => String::from("NOP"),
        0x21 => {
            num_bytes = 3;

            format!(
                "LXI\tH, {:02x}{:02x}",
                code_buffer[pc + 2],
                code_buffer[pc + 1]
            )
        }
        0x22 => {
            num_bytes = 3;

            format!(
                "SHLD\t${:02x}{:02x}",
                code_buffer[pc + 2],
                code_buffer[pc + 1]
            )
        }
        0x23 => String::from("INX\tH"),
        0x24 => String::from("INR\tH"),
        0x25 => String::from("DCR\tH"),
        0x26 => {
            num_bytes = 2;
            format!("MVI\tH,{:02x}", code_buffer[pc + 1])
        }
        0x27 => String::from("DAA"),
        0x28 => String::from("NOP"),
        0x29 => String::from("DAD\tH"),
        0x2a => {
            num_bytes = 3;

            format!(
                "LHLD\t${:02x}{:02x}",
                code_buffer[pc + 2],
                code_buffer[pc + 1]
            )
        }
        0x2b => String::from("DCX\tH"),
        0x2c => String::from("INR\tL"),
        0x2d => String::from("DCR\tL"),
        0x2e => {
            num_bytes = 2;
            format!("MVI\tL,{:02x}", code_buffer[pc + 1])
        }
        0x2f => String::from("CMA"),
        0x30 => String::from("NOP"),
        0x31 => {
            num_bytes = 3;

            format!(
                "LXI\tSP,{:02x}{:02x}",
                code_buffer[pc + 2],
                code_buffer[pc + 1]
            )
        }
        0x32 => {
            num_bytes = 3;

            format!(
                "STA\t{:02x}{:02x}",
                code_buffer[pc + 2],
                code_buffer[pc + 1]
            )
        }
        0x33 => String::from("INX\tSP"),
        0x34 => String::from("INR\tM"),
        0x35 => String::from("DCR\tM"),
        0x36 => {
            num_bytes = 2;
            format!("MVI\tM,{:02x}", code_buffer[pc + 1])
        }
        0x37 => String::from("STC"),
        0x38 => String::from("NOP"),
        0x39 => String::from("DAD\tSP"),
        0x3a => {
            num_bytes = 3;

            format!(
                "LDA\t{:02x}{:02x}",
                code_buffer[pc + 2],
                code_buffer[pc + 1]
            )
        }
        0x3b => String::from("DCX\tSP"),
        0x3c => String::from("INR\tA"),
        0x3d => String::from("DCR\tA"),
        0x3e => {
            num_bytes = 2;
            format!("MVI\tA,{:02x}", code_buffer[pc + 1])
        }
        0x3f => String::from("CMC"),
        0x40 => String::from("MOV\tB,B"),
        0x41 => String::from("MOV\tB,C"),
        0x42 => String::from("MOV\tB,D"),
        0x43 => String::from("MOV\tB,E"),
        0x44 => String::from("MOV\tB,H"),
        0x45 => String::from("MOV\tB,L"),
        0x46 => String::from("MOV\tB,M"),
        0x47 => String::from("MOV\tB,A"),
        0x48 => String::from("MOV\tC,B"),
        0x49 => String::from("MOV\tC,C"),
        0x4a => String::from("MOV\tC,D"),
        0x4b => String::from("MOV\tC,E"),
        0x4c => String::from("MOV\tC,H"),
        0x4d => String::from("MOV\tC,L"),
        0x4e => String::from("MOV\tC,M"),
        0x4f => String::from("MOV\tC,A"),
        0x50 => String::from("MOV\tD,B"),
        0x51 => String::from("MOV\tD,C"),
        0x52 => String::from("MOV\tD,D"),
        0x53 => String::from("MOV\tD,E"),
        0x54 => String::from("MOV\tD,H"),
        0x55 => String::from("MOV\tD,L"),
        0x56 => String::from("MOV\tD,M"),
        0x57 => String::from("MOV\tD,A"),
        0x58 => String::from("MOV\tE,B"),
        0x59 => String::from("MOV\tE,C"),
        0x5a => String::from("MOV\tE,D"),
        0x5b => String::from("MOV\tE,E"),
        0x5c => String::from("MOV\tE,H"),
        0x5d => String::from("MOV\tE,L"),
        0x5e => String::from("MOV\tE,M"),
        0x5f => String::from("MOV\tE,A"),

        0x60 => String::from("MOV\tH,B"),
        0x61 => String::from("MOV\tH,C"),
        0x62 => String::from("MOV\tH,D"),
        0x63 => String::from("MOV\tH,E"),
        0x64 => String::from("MOV\tH,H"),
        0x65 => String::from("MOV\tH,L"),
        0x66 => String::from("MOV\tH,M"),
        0x67 => String::from("MOV\tH,A"),
        0x68 => String::from("MOV\tL,B"),
        0x69 => String::from("MOV\tL,C"),
        0x6a => String::from("MOV\tL,D"),
        0x6b => String::from("MOV\tL,E"),
        0x6c => String::from("MOV\tL,H"),
        0x6d => String::from("MOV\tL,L"),
        0x6e => String::from("MOV\tL,M"),
        0x6f => String::from("MOV\tL,A"),

        0x70 => String::from("MOV\tM,B"),
        0x71 => String::from("MOV\tM,C"),
        0x72 => String::from("MOV\tM,D"),
        0x73 => String::from("MOV\tM,E"),
        0x74 => String::from("MOV\tM,H"),
        0x75 => String::from("MOV\tM,L"),
        0x76 => String::from("HLT"),
        0x77 => String::from("MOV\tM,A"),
        0x78 => String::from("MOV\tA,B"),
        0x79 => String::from("MOV\tA,C"),
        0x7a => String::from("MOV\tA,D"),
        0x7b => String::from("MOV\tA,E"),
        0x7c => String::from("MOV\tA,H"),
        0x7d => String::from("MOV\tA,L"),
        0x7e => String::from("MOV\tA,M"),
        0x7f => String::from("MOV\tA,A"),

        0x80 => String::from("ADD\tB"),
        0x81 => String::from("ADD\tC"),
        0x82 => String::from("ADD\tD"),
        0x83 => String::from("ADD\tE"),
        0x84 => String::from("ADD\tH"),
        0x85 => String::from("ADD\tL"),
        0x86 => String::from("ADD\tM"),
        0x87 => String::from("ADD\tA"),
        0x88 => String::from("ADC\tB"),
        0x89 => String::from("ADC\tC"),
        0x8a => String::from("ADC\tD"),
        0x8b => String::from("ADC\tE"),
        0x8c => String::from("ADC\tH"),
        0x8d => String::from("ADC\tL"),
        0x8e => String::from("ADC\tM"),
        0x8f => String::from("ADC\tA"),

        0x90 => String::from("SUB\tB"),
        0x91 => String::from("SUB\tC"),
        0x92 => String::from("SUB\tD"),
        0x93 => String::from("SUB\tE"),
        0x94 => String::from("SUB\tH"),
        0x95 => String::from("SUB\tL"),
        0x96 => String::from("SUB\tM"),
        0x97 => String::from("SUB\tA"),
        0x98 => String::from("SBB\tB"),
        0x99 => String::from("SBB\tC"),
        0x9a => String::from("SBB\tD"),
        0x9b => String::from("SBB\tE"),
        0x9c => String::from("SBB\tH"),
        0x9d => String::from("SBB\tL"),
        0x9e => String::from("SBB\tM"),
        0x9f => String::from("SBB\tA"),

        0xa0 => String::from("ANA\tB"),
        0xa1 => String::from("ANA\tC"),
        0xa2 => String::from("ANA\tD"),
        0xa3 => String::from("ANA\tE"),
        0xa4 => String::from("ANA\tH"),
        0xa5 => String::from("ANA\tL"),
        0xa6 => String::from("ANA\tM"),
        0xa7 => String::from("ANA\tA"),
        0xa8 => String::from("XRA\tB"),
        0xa9 => String::from("XRA\tC"),
        0xaa => String::from("XRA\tD"),
        0xab => String::from("XRA\tE"),
        0xac => String::from("XRA\tH"),
        0xad => String::from("XRA\tL"),
        0xae => String::from("XRA\tM"),
        0xaf => String::from("XRA\tA"),

        0xb0 => String::from("ORA\tB"),
        0xb1 => String::from("ORA\tC"),
        0xb2 => String::from("ORA\tD"),
        0xb3 => String::from("ORA\tE"),
        0xb4 => String::from("ORA\tH"),
        0xb5 => String::from("ORA\tL"),
        0xb6 => String::from("ORA\tM"),
        0xb7 => String::from("ORA\tA"),
        0xb8 => String::from("CMP\tB"),
        0xb9 => String::from("CMP\tC"),
        0xba => String::from("CMP\tD"),
        0xbb => String::from("CMP\tE"),
        0xbc => String::from("CMP\tH"),
        0xbd => String::from("CMP\tL"),
        0xbe => String::from("CMP\tM"),
        0xbf => String::from("CMP\tA"),

        0xc0 => String::from("RNZ"),
        0xc1 => String::from("POP\tB"),
        0xc2 => {
            num_bytes = 3;

            format!(
                "JNZ\t{:02x}{:02x}",
                code_buffer[pc + 2],
                code_buffer[pc + 1]
            )
        }
        0xc3 => {
            num_bytes = 3;

            format!(
                "JMP\t{:02x}{:02x}",
                code_buffer[pc + 2],
                code_buffer[pc + 1]
            )
        }
        0xc4 => {
            num_bytes = 3;

            format!(
                "CNZ\t{:02x}{:02x}",
                code_buffer[pc + 2],
                code_buffer[pc + 1]
            )
        }
        0xc5 => String::from("PUSH\tB"),
        0xc6 => {
            num_bytes = 2;
            format!("ADI\t{:02x}", code_buffer[pc + 1])
        }
        0xc7 => String::from("RST\t0"),
        0xc8 => String::from("RZ"),
        0xc9 => String::from("RET"),
        0xca => {
            num_bytes = 2;
            format!("JZ\t{:02x}{:02x}", code_buffer[pc + 2], code_buffer[pc + 1])
        }
        0xcb => {
            num_bytes = 3;

            format!(
                "JMP\t{:02x}{:02x}",
                code_buffer[pc + 2],
                code_buffer[pc + 1]
            )
        }
        0xcc => {
            num_bytes = 3;

            format!("CZ\t{:02x}{:02x}", code_buffer[pc + 2], code_buffer[pc + 1])
        }
        0xcd => {
            num_bytes = 3;

            format!(
                "CALL\t{:02x}{:02x}",
                code_buffer[pc + 2],
                code_buffer[pc + 1]
            )
        }
        0xce => {
            num_bytes = 2;
            format!("ACI\t{:02x}", code_buffer[pc + 1])
        }
        0xcf => String::from("RST\t1"),

        0xd0 => String::from("RNC"),
        0xd1 => String::from("POP\tD"),
        0xd2 => {
            num_bytes = 3;

            format!(
                "JNC\t{:02x}{:02x}",
                code_buffer[pc + 2],
                code_buffer[pc + 1]
            )
        }
        0xd3 => {
            num_bytes = 2;
            format!("OUT\t{:02x}", code_buffer[pc + 1])
        }
        0xd4 => {
            num_bytes = 3;

            format!(
                "CNC\t{:02x}{:02x}",
                code_buffer[pc + 2],
                code_buffer[pc + 1]
            )
        }
        0xd5 => String::from("PUSH\tD"),
        0xd6 => {
            num_bytes = 2;
            format!("SUI\t{:02x}", code_buffer[pc + 1])
        }
        0xd7 => String::from("RST\t2"),
        0xd8 => String::from("RC"),
        0xd9 => String::from("RET"),
        0xda => {
            num_bytes = 3;

            format!("JC\t{:02x}{:02x}", code_buffer[pc + 2], code_buffer[pc + 1])
        }
        0xdb => {
            num_bytes = 2;
            format!("IN\t{:02x}", code_buffer[pc + 1])
        }
        0xdc => {
            num_bytes = 3;

            format!("CC\t{:02x}{:02x}", code_buffer[pc + 2], code_buffer[pc + 1])
        }
        0xdd => {
            num_bytes = 3;

            format!(
                "CALL\t{:02x}{:02x}",
                code_buffer[pc + 2],
                code_buffer[pc + 1]
            )
        }
        0xde => {
            num_bytes = 2;
            format!("SBI\t{:02x}", code_buffer[pc + 1])
        }
        0xdf => String::from("RST\t3"),

        0xe0 => String::from("RPO"),
        0xe1 => String::from("POP\tH"),
        0xe2 => {
            num_bytes = 3;

            format!(
                "JPO\t{:02x}{:02x}",
                code_buffer[pc + 2],
                code_buffer[pc + 1]
            )
        }
        0xe3 => String::from("XTHL"),
        0xe4 => {
            num_bytes = 3;

            format!(
                "CPO\t{:02x}{:02x}",
                code_buffer[pc + 2],
                code_buffer[pc + 1]
            )
        }
        0xe5 => String::from("PUSH\tH"),
        0xe6 => {
            num_bytes = 2;
            format!("ANI\t{:02x}", code_buffer[pc + 1])
        }
        0xe7 => String::from("RST\t4"),
        0xe8 => String::from("RPE"),
        0xe9 => String::from("PCHL"),
        0xea => {
            num_bytes = 3;

            format!(
                "JPE\t{:02x}{:02x}",
                code_buffer[pc + 2],
                code_buffer[pc + 1]
            )
        }
        0xeb => String::from("XCHG"),
        0xec => {
            num_bytes = 3;

            format!(
                "CPE\t{:02x}{:02x}",
                code_buffer[pc + 2],
                code_buffer[pc + 1]
            )
        }
        0xed => {
            num_bytes = 3;

            format!(
                "CALL\t{:02x}{:02x}",
                code_buffer[pc + 2],
                code_buffer[pc + 1]
            )
        }
        0xee => {
            num_bytes = 2;
            format!("XRI\t{:02x}", code_buffer[pc + 1])
        }
        0xef => String::from("RST\t5"),

        0xf0 => String::from("RP"),
        0xf1 => String::from("POP\tPSW"),
        0xf2 => {
            num_bytes = 3;

            format!("JP\t{:02x}{:02x}", code_buffer[pc + 2], code_buffer[pc + 1])
        }
        0xf3 => String::from("DI"),
        0xf4 => {
            num_bytes = 3;

            format!("CP\t{:02x}{:02x}", code_buffer[pc + 2], code_buffer[pc + 1])
        }
        0xf5 => String::from("PUSH PSW"),
        0xf6 => {
            num_bytes = 2;
            format!("ORI\t{:02x}", code_buffer[pc + 1])
        }
        0xf7 => String::from("RST\t6"),
        0xf8 => String::from("RM"),
        0xf9 => String::from("SPHL"),
        0xfa => {
            num_bytes = 3;

            format!("JM\t{:02x}{:02x}", code_buffer[pc + 2], code_buffer[pc + 1])
        }
        0xfb => String::from("EI"),
        0xfc => {
            num_bytes = 3;

            format!("CM\t{:02x}{:02x}", code_buffer[pc + 2], code_buffer[pc + 1])
        }
        0xfd => {
            num_bytes = 3;

            format!(
                "CALL\t{:02x}{:02x}",
                code_buffer[pc + 2],
                code_buffer[pc + 1]
            )
        }
        0xfe => {
            num_bytes = 2;

            format!("CPI\t{:02x}", code_buffer[pc + 1])
        }
        0xff => String::from("RST 7"),
        number => String::from("Other"),
    };
    println!("{:04x}\t{}", pc, code);
    num_bytes
}

struct ConditionFlags {
    zero: bool,
    sign: bool,
    parity: bool,
    carry: bool,
    aux_carry: bool,
}
struct State8080 {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
    condition_flags: ConditionFlags,
    memory: Vec<u8>,
    int_enable: u8,
}

fn instruction_not_implemented(state: &mut State8080) {
    println!("Instruction not implemented");
}

fn emulate_8080(state: &mut State8080) {
    let op_code: u8 = state.memory[state.pc as usize];
    match op_code {
        0x00 => instruction_not_implemented(state),
        0x01 => {
            state.c = state.memory[state.pc as usize + 1];
            state.b = state.memory[state.pc as usize + 2];
            state.pc += 3;
        }
        0x02 => instruction_not_implemented(state),
        0x03 => instruction_not_implemented(state),
        0x04 => instruction_not_implemented(state),
        0x05 => {
            let res = state.b - 1;
            state.condition_flags.zero = res == 0;
            state.condition_flags.sign = 0x80 == (res & 0x80);
            state.condition_flags.parity = parity(res, 8) as bool;
            state.b = res;
        }

        _ => instruction_not_implemented(state),
    }
}
fn parity(mut x: u8, size: u8) -> bool {
    let mut i = 0;
    let mut p = 0;
    x = x & ((1 << size) - 1);
    while i < size {
        if x & 0x1 == 1 {
            p += 1;
        }
        x = x >> 1;
        i += 1;
    }
    (0 == (p & 0x1))
}
