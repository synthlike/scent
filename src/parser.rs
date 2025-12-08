use std::fmt;

#[derive(PartialEq)]
pub struct Instruction {
    pub offset: usize,
    pub opcode: u8,
    pub data: Vec<u8>, // for push
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Instruction {{ offset: {}, opcode: 0x{:02x}, data: 0x{} }}",
            self.offset,
            self.opcode,
            hex::encode(&self.data)
        )
    }
}

pub fn parse_bytecode(bytes: Vec<u8>) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let mut i = 0;

    let bytes = strip_solc_metadata(&bytes);

    while i < bytes.len() {
        let offset = i;
        let opcode = bytes[i];
        i += 1;

        // push1 = 0x60, push32 = 0x7F
        let data = if opcode >= 0x60 && opcode <= 0x7F {
            let push_size = (opcode - 0x5F) as usize;
            let mut push_data = Vec::new();

            for _ in 0..push_size {
                if i < bytes.len() {
                    push_data.push(bytes[i]);
                    i += 1;
                }
            }
            push_data
        } else {
            Vec::new()
        };

        instructions.push(Instruction {
            offset,
            opcode,
            data,
        });
    }

    instructions
}

pub fn opcode_name(opcode: u8) -> String {
    // from evm.codes
    match opcode {
        0x00 => String::from("STOP"),
        0x15 => String::from("ISZERO"),
        0x34 => String::from("CALLVALUE"),
        0x50 => String::from("POP"),
        0x52 => String::from("MSTORE"),
        0x57 => String::from("JUMPI"),
        0x5B => String::from("JUMPDEST"),
        0x5f => String::from("PUSH0"),
        0x5F..=0x7F => String::from("PUSH"),
        0x80 => String::from("DUP1"),
        0xFD => String::from("REVERT"),
        0x08 => String::from("ADDMOD"),
        0x0A => String::from("EXP"),
        0x39 => String::from("CODECOPY"),
        0xA1 => String::from("LOG1"),
        0xF3 => String::from("RETURN"),
        0xFE => String::from("INVALID"),
        byte => format!("UNKNOWN(0x{:02x})", byte),
    }
}

pub fn print_instruction(inst: &Instruction) {
    print!("{:04x}: {}", inst.offset, opcode_name(inst.opcode));

    if !inst.data.is_empty() {
        print!("{} 0x", inst.opcode - 0x5f);
        for byte in &inst.data {
            print!("{:02x}", byte);
        }
    }
    println!()
}

fn strip_solc_metadata(bytecode: &[u8]) -> &[u8] {
    if let Some(pos) = bytecode
        .windows(7)
        .rposition(|w| w == b"\xa1\x64\x73\x6f\x6c\x63\x43")
    {
        &bytecode[..pos]
    } else {
        bytecode
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex;

    #[test]
    fn parse() {
        let input: Vec<u8> = hex::decode("6080604052348015600e575f5ffd5b50601580601a5f395ff3fe60806040525f5ffdfea164736f6c634300081e000a").expect("invalid hex"); // empty.sol
        let bytecode = parse_bytecode(input);

        assert_eq!(
            bytecode[0],
            Instruction {
                offset: 0,
                opcode: 0x60,
                data: vec![0x80]
            }
        );
        assert_eq!(
            bytecode[1],
            Instruction {
                offset: 2,
                opcode: 0x60,
                data: vec![0x40]
            }
        );
        assert_eq!(
            bytecode[2],
            Instruction {
                offset: 4,
                opcode: 0x52,
                data: Vec::new()
            }
        );
        assert_eq!(
            bytecode[3],
            Instruction {
                offset: 5,
                opcode: 0x34,
                data: Vec::new()
            }
        );
        // ... and so on
    }

    // must be run via cargo test -- --nocapture
    #[test]
    fn print() {
        let input: Vec<u8> = hex::decode("6080604052348015600e575f5ffd5b50601580601a5f395ff3fe60806040525f5ffdfea164736f6c634300081e000a").expect("invalid hex"); // empty.sol
        let bytecode = parse_bytecode(input);
        for inst in bytecode {
            print_instruction(&inst);
        }
    }
}
