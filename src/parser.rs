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
        0x01 => String::from("ADD"),
        0x02 => String::from("MUL"),
        0x03 => String::from("SUB"),
        0x04 => String::from("DIV"),
        0x05 => String::from("SDIV"),
        0x06 => String::from("MOD"),
        0x07 => String::from("SMOD"),
        0x08 => String::from("ADDMOD"),
        0x09 => String::from("MULMOD"),
        0x0A => String::from("EXP"),
        0x10 => String::from("LT"),
        0x11 => String::from("GT"),
        0x12 => String::from("SLT"),
        0x13 => String::from("GLT"),
        0x14 => String::from("EQ"),
        0x15 => String::from("ISZERO"),
        0x1A => String::from("BYTE"),
        0x1B => String::from("SHL"),
        0x1C => String::from("SHR"),
        0x1D => String::from("SAR"),
        0x20 => String::from("KECCAK256"),
        0x34 => String::from("CALLVALUE"),
        0x35 => String::from("CALLDATALOAD"),
        0x36 => String::from("CALLDATASIZE"),
        0x37 => String::from("CALLDATACOPY"),
        0x38 => String::from("CODESIZE"),
        0x39 => String::from("CODECOPY"),
        // TODO: 0x3A - 0x4A
        0x50 => String::from("POP"),
        0x51 => String::from("MLOAD"),
        0x52 => String::from("MSTORE"),
        0x53 => String::from("MSTORES"),
        0x54 => String::from("SLOAD"),
        0x55 => String::from("SSTORE"),
        0x56 => String::from("JUMP"),
        0x57 => String::from("JUMPI"),
        0x5B => String::from("JUMPDEST"),
        0x5C => String::from("TLOAD"),
        0x5D => String::from("TSTORE"),
        0x5E => String::from("MCOPY"),
        0x5F => String::from("PUSH0"),
        0x60..=0x7F => String::from("PUSH"),
        0x80 => String::from("DUP1"),
        0x81 => String::from("DUP2"),
        0x82 => String::from("DUP3"),
        0x83 => String::from("DUP4"),
        0x84 => String::from("DUP5"),
        0x85 => String::from("DUP6"),
        0x86 => String::from("DUP7"),
        0x87 => String::from("DUP8"),
        0x88 => String::from("DUP9"),
        0x89 => String::from("DUP10"),
        0x8A => String::from("DUP11"),
        0x8B => String::from("DUP12"),
        0x8C => String::from("DUP13"),
        0x8D => String::from("DUP14"),
        0x8E => String::from("DUP15"),
        0x8F => String::from("DUP16"),
        0x90 => String::from("SWAP1"),
        0x91 => String::from("SWAP2"),
        0x92 => String::from("SWAP3"),
        0x93 => String::from("SWAP4"),
        0x94 => String::from("SWAP5"),
        0x95 => String::from("SWAP6"),
        0x96 => String::from("SWAP7"),
        0x97 => String::from("SWAP8"),
        0x98 => String::from("SWAP9"),
        0x99 => String::from("SWAP10"),
        0x9A => String::from("SWAP11"),
        0x9B => String::from("SWAP12"),
        0x9C => String::from("SWAP13"),
        0x9D => String::from("SWAP14"),
        0x9E => String::from("SWAP15"),
        0x9F => String::from("SWAP16"),
        0xA1 => String::from("LOG1"),
        0xA2 => String::from("LOG2"),
        0xA3 => String::from("LOG3"),
        0xA4 => String::from("LOG4"),
        0xF0 => String::from("CREATE"),
        0xF1 => String::from("CALL"),
        0xF2 => String::from("CALLCODE"),
        0xF3 => String::from("RETURN"),
        0xF4 => String::from("DELEGATECALL"),
        0xF5 => String::from("CREATE2"),
        0xFA => String::from("STATICCALL"),
        0xFD => String::from("REVERT"),
        0xFE => String::from("INVALID"),

        byte => format!("UNKNOWN(0x{:02x})", byte),
    }
}

pub fn print_instruction(inst: &Instruction) {
    print!(
        "{:04x} {:<3}  {}",
        inst.offset,
        inst.opcode,
        opcode_name(inst.opcode)
    );

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
    fn empty() {
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
    fn print_empty_contract_bytecode() {
        let input: Vec<u8> = hex::decode("6080604052348015600e575f5ffd5b50601580601a5f395ff3fe60806040525f5ffdfea164736f6c634300081e000a").expect("invalid hex"); // empty.sol
        let bytecode = parse_bytecode(input);
        for inst in bytecode {
            print_instruction(&inst);
        }
    }

    #[test]
    fn print_return_constant_bytecode() {
        let input: Vec<u8> = hex::decode("6080604052348015600e575f5ffd5b50608680601a5f395ff3fe6080604052348015600e575f5ffd5b50600436106026575f3560e01c80632096525514602a575b5f5ffd5b60306044565b604051603b91906062565b60405180910390f35b5f602a905090565b5f819050919050565b605c81604c565b82525050565b5f60208201905060735f8301846055565b9291505056fea164736f6c634300081e000a").expect("invalid hex"); // return_const.sol
        let bytecode = parse_bytecode(input);
        for inst in bytecode {
            print_instruction(&inst);
        }
    }

    #[test]
    fn print_counter_bytecode() {
        let input: Vec<u8> = hex::decode("6080604052348015600e575f5ffd5b506101b88061001c5f395ff3fe608060405234801561000f575f5ffd5b506004361061003f575f3560e01c80633fb5c1cb146100435780638381f58a1461005f578063d09de08a1461007d575b5f5ffd5b61005d600480360381019061005891906100e4565b610087565b005b610067610090565b604051610074919061011e565b60405180910390f35b610085610095565b005b805f8190555050565b5f5481565b5f5f8154809291906100a690610164565b9190505550565b5f5ffd5b5f819050919050565b6100c3816100b1565b81146100cd575f5ffd5b50565b5f813590506100de816100ba565b92915050565b5f602082840312156100f9576100f86100ad565b5b5f610106848285016100d0565b91505092915050565b610118816100b1565b82525050565b5f6020820190506101315f83018461010f565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f61016e826100b1565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82036101a05761019f610137565b5b60018201905091905056fea164736f6c634300081e000a").expect("invalid hex"); // counter.sol
        let bytecode = parse_bytecode(input);
        for inst in bytecode {
            print_instruction(&inst);
        }
    }
}
