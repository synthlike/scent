use crate::parser::Instruction;

pub struct View {
    pub entries: Vec<ViewEntry>,
}

pub enum ViewEntry {
    Instruction(Instruction),
    InstructionWithComment(Instruction, String, CommentPlacement),
    Label(String),
}

pub enum CommentPlacement {
    Above,
    NextTo,
}

impl View {
    pub fn from_instructions(instructions: &[Instruction], decorated: bool) -> Self {
        let mut entries = Vec::new();

        for inst in instructions {
            if decorated && is_push_instruction(inst.opcode) {
                let comment = decode_push_value(&inst.data);
                entries.push(ViewEntry::InstructionWithComment(
                    inst.clone(),
                    comment,
                    CommentPlacement::NextTo,
                ));
            } else {
                entries.push(ViewEntry::Instruction(inst.clone()))
            }
        }

        Self { entries }
    }

    pub fn print_entries(self) {
        for entry in self.entries {
            match entry {
                ViewEntry::Instruction(inst) => {
                    println!("{}", format_instruction(&inst));
                }
                ViewEntry::InstructionWithComment(inst, comment, placement) => match placement {
                    CommentPlacement::Above => {
                        println!("; {}", comment);
                        println!("{}", format_instruction(&inst));
                    }
                    CommentPlacement::NextTo => {
                        println!("{} ; {}", format_instruction(&inst), comment);
                    }
                },
                ViewEntry::Label(_) => todo!(),
            }
        }
    }
}

fn decode_push_value(data: &[u8]) -> String {
    if data.is_empty() {
        return "0".to_string();
    }

    let mut value: u128 = 0;

    // up to 16 bytes (u128)
    if data.len() < 16 {
        for &byte in data {
            value = value * 256 + byte as u128;
        }
        format!("{}", value)
    } else {
        return "too large value".to_string();
    }
}

fn is_push_instruction(opcode: u8) -> bool {
    opcode >= 0x60 && opcode <= 0x7F
}

fn format_instruction(inst: &Instruction) -> String {
    let mut output = format!(
        "{:04x} {:<3x}  {}",
        inst.offset,
        inst.opcode,
        opcode_name(inst.opcode)
    );

    if !inst.data.is_empty() {
        output.push_str(&format!(
            "{} 0x{}",
            inst.opcode - 0x5f,
            hex::encode(&inst.data)
        ));
    }

    output
}

pub fn opcode_name(opcode: u8) -> &'static str {
    // from evm.codes
    match opcode {
        0x00 => "STOP",
        0x01 => "ADD",
        0x02 => "MUL",
        0x03 => "SUB",
        0x04 => "DIV",
        0x05 => "SDIV",
        0x06 => "MOD",
        0x07 => "SMOD",
        0x08 => "ADDMOD",
        0x09 => "MULMOD",
        0x0A => "EXP",
        0x0B => "SIGNEXTEND",
        0x10 => "LT",
        0x11 => "GT",
        0x12 => "SLT",
        0x13 => "SGT",
        0x14 => "EQ",
        0x15 => "ISZERO",
        0x16 => "AND",
        0x17 => "OR",
        0x18 => "XOR",
        0x19 => "NOT",
        0x1A => "BYTE",
        0x1B => "SHL",
        0x1C => "SHR",
        0x1D => "SAR",
        0x20 => "KECCAK256",
        0x30 => "ADDRESS",
        0x31 => "BALANCE",
        0x32 => "ORIGIN",
        0x33 => "CALLER",
        0x34 => "CALLVALUE",
        0x35 => "CALLDATALOAD",
        0x36 => "CALLDATASIZE",
        0x37 => "CALLDATACOPY",
        0x38 => "CODESIZE",
        0x39 => "CODECOPY",
        0x3A => "GASPRICE",
        0x3B => "EXTCODESIZE",
        0x3C => "EXTCODECOPY",
        0x3D => "RETURNDATASIZE",
        0x3E => "RETURNDATACOPY",
        0x3F => "EXTCODEHASH",
        0x40 => "BLOCKHASH",
        0x41 => "COINBASE",
        0x42 => "TIMESTAMP",
        0x43 => "NUMBER",
        0x44 => "DIFFICULTY",
        0x45 => "GASLIMIT",
        0x46 => "CHAINID",
        0x47 => "SELFBALANCE",
        0x48 => "BASEFEE",
        0x49 => "BLOBHASH",
        0x4A => "BLOBBASEFEE",
        0x50 => "POP",
        0x51 => "MLOAD",
        0x52 => "MSTORE",
        0x53 => "MSTORE8",
        0x54 => "SLOAD",
        0x55 => "SSTORE",
        0x56 => "JUMP",
        0x57 => "JUMPI",
        0x58 => "PC",
        0x59 => "MSIZE",
        0x5A => "GAS",
        0x5B => "JUMPDEST",
        0x5C => "TLOAD",
        0x5D => "TSTORE",
        0x5E => "MCOPY",
        0x5F => "PUSH0",
        0x60..=0x7F => "PUSH",
        0x80 => "DUP1",
        0x81 => "DUP2",
        0x82 => "DUP3",
        0x83 => "DUP4",
        0x84 => "DUP5",
        0x85 => "DUP6",
        0x86 => "DUP7",
        0x87 => "DUP8",
        0x88 => "DUP9",
        0x89 => "DUP10",
        0x8A => "DUP11",
        0x8B => "DUP12",
        0x8C => "DUP13",
        0x8D => "DUP14",
        0x8E => "DUP15",
        0x8F => "DUP16",
        0x90 => "SWAP1",
        0x91 => "SWAP2",
        0x92 => "SWAP3",
        0x93 => "SWAP4",
        0x94 => "SWAP5",
        0x95 => "SWAP6",
        0x96 => "SWAP7",
        0x97 => "SWAP8",
        0x98 => "SWAP9",
        0x99 => "SWAP10",
        0x9A => "SWAP11",
        0x9B => "SWAP12",
        0x9C => "SWAP13",
        0x9D => "SWAP14",
        0x9E => "SWAP15",
        0x9F => "SWAP16",
        0xA0 => "LOG0",
        0xA1 => "LOG1",
        0xA2 => "LOG2",
        0xA3 => "LOG3",
        0xA4 => "LOG4",
        0xF0 => "CREATE",
        0xF1 => "CALL",
        0xF2 => "CALLCODE",
        0xF3 => "RETURN",
        0xF4 => "DELEGATECALL",
        0xF5 => "CREATE2",
        0xFA => "STATICCALL",
        0xFD => "REVERT",
        0xFE => "INVALID",
        _ => "UNKNOWN",
    }
}
