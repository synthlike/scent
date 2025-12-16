use std::fmt;

use crate::parser::Instruction;

pub struct Analysis {
    pub function_selectors: Vec<FunctionSelector>,
    pub functions: Vec<Function>,
    pub function_entrypoints: Vec<FunctionEntrypoint>,
}

impl Analysis {
    pub fn new() -> Self {
        Self {
            function_selectors: Vec::new(),
            functions: Vec::new(),
            function_entrypoints: Vec::new(),
        }
    }

    pub fn from_instructions(instructions: &[Instruction]) -> Self {
        Self {
            function_selectors: analyze_function_selectors(instructions),
            functions: analyze_functions(instructions),
            function_entrypoints: analyze_function_entrypoints(instructions),
        }
    }
}

#[derive(PartialEq)]
pub struct FunctionSelector {
    pub offset: usize,
    pub selector: [u8; 4],
}

impl fmt::Debug for FunctionSelector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Selector {{ offset: {}, selector: 0x{} }}",
            self.offset,
            hex::encode(&self.selector),
        )
    }
}

pub fn analyze_function_selectors(instructions: &[Instruction]) -> Vec<FunctionSelector> {
    instructions
        .windows(2)
        .filter_map(|w| {
            let first = &w[0];
            let second = &w[1];

            // PUSH4 <selector>
            if first.opcode == 0x63 && first.data.len() == 4 &&
            // EQ
            second.opcode == 0x14
            {
                let mut selector = [0u8; 4];
                selector.copy_from_slice(&first.data);

                Some(FunctionSelector {
                    offset: first.offset,
                    selector,
                })
            } else {
                None
            }
        })
        .collect()
}

pub struct Function {
    pub selector: [u8; 4],
    pub start: usize,
    pub end: usize,
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Function {{ selector: 0x{}, start: 0x{:04x}, end: 0x{:04x} }}",
            hex::encode(&self.selector),
            self.start,
            self.end,
        )
    }
}

pub fn analyze_functions(instructions: &[Instruction]) -> Vec<Function> {
    instructions
        .windows(4)
        .filter_map(|w| {
            let first = &w[0];
            let second = &w[1];
            let third = &w[2];
            let fourth = &w[3];

            // PUSH4
            if first.opcode == 0x63 && first.data.len() == 4 &&
            // EQ
            second.opcode == 0x14 &&
            // PUSH1/PUSH2/PUSH3
            third.opcode >= 0x60 && third.opcode <= 0x62 &&
            // JUMPI
            fourth.opcode == 0x57
            {
                let mut selector = [0u8; 4];
                selector.copy_from_slice(&first.data);

                let start = bytes_to_usize(&third.data);
                let end = find_function_end(instructions, start)?;

                return Some(Function {
                    selector,
                    start,
                    end,
                });
            }

            None
        })
        .collect()
}

pub struct FunctionEntrypoint {
    pub selector: [u8; 4],
    pub offset: usize,
}

impl fmt::Debug for FunctionEntrypoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Function {{ selector: 0x{}, offset: 0x{:04x} }}",
            hex::encode(&self.selector),
            self.offset,
        )
    }
}

pub fn analyze_function_entrypoints(instructions: &[Instruction]) -> Vec<FunctionEntrypoint> {
    instructions
        .windows(4)
        .filter_map(|w| {
            let first = &w[0];
            let second = &w[1];
            let third = &w[2];
            let fourth = &w[3];

            // PUSH4
            if first.opcode == 0x63 && first.data.len() == 4 &&
            // EQ
            second.opcode == 0x14 &&
            // PUSH1/PUSH2/PUSH3
            third.opcode >= 0x60 && third.opcode <= 0x62 &&
            // JUMPI
            fourth.opcode == 0x57
            {
                let mut selector = [0u8; 4];
                selector.copy_from_slice(&first.data);

                let offset = bytes_to_usize(&third.data);

                return Some(FunctionEntrypoint { selector, offset });
            }

            None
        })
        .collect()
}

fn find_function_end(instructions: &[Instruction], start_offset: usize) -> Option<usize> {
    let start_idx = instructions.iter().position(|i| i.offset == start_offset)?;

    for i in start_idx..instructions.len() {
        let inst = &instructions[i];

        // RETURN, REVERT, STOP, INVALID
        if matches!(inst.opcode, 0xF3 | 0xFD | 0x00 | 0xFE) {
            return Some(inst.offset);
        }

        // JUMP (likely to shared return logic)
        if inst.opcode == 0x56 {
            // TODO: if forward jump that migt be still within function
            // return only on backward jumps?
            return Some(inst.offset);
        }

        // JUMPDEST (to next func)
        // make sure we are not at the start_idx jumpdest
        if inst.opcode == 0x5B && i > start_idx {
            return Some(instructions[i - 1].offset);
        }
    }

    instructions.last().map(|inst| inst.offset)
}

fn bytes_to_usize(bytes: &[u8]) -> usize {
    let mut result = 0;
    for &byte in bytes {
        result = (result << 8) | byte as usize;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn analyze_function_selector() {
        let instructions = vec![
            Instruction {
                offset: 100,
                opcode: 0x63,
                data: vec![0x11, 0x22, 0x33, 0x44],
            },
            Instruction {
                offset: 106,
                opcode: 0x14,
                data: Vec::new(),
            },
        ];

        let analysis = analyze_function_selectors(&instructions);

        assert_eq!(analysis.len(), 1);
        assert_eq!(
            analysis[0],
            FunctionSelector {
                offset: 100,
                selector: [0x11, 0x22, 0x33, 0x44],
            }
        )
    }

    #[test]
    fn analyze_function() {
        let instructions = vec![
            Instruction {
                offset: 0x00,
                opcode: 0x63,
                data: vec![0xa9, 0x05, 0x9c, 0xbb],
            }, // PUSH4 selector
            Instruction {
                offset: 0x05,
                opcode: 0x14,
                data: vec![],
            }, // EQ
            Instruction {
                offset: 0x06,
                opcode: 0x61,
                data: vec![0x02, 0x34],
            }, // PUSH2 0x0234
            Instruction {
                offset: 0x09,
                opcode: 0x57,
                data: vec![],
            }, // JUMPI
            Instruction {
                offset: 0x234,
                opcode: 0x5B,
                data: vec![],
            }, // JUMPDEST
            Instruction {
                offset: 0x235,
                opcode: 0x34,
                data: vec![],
            }, // CALLVALUE
            Instruction {
                offset: 0x236,
                opcode: 0xF3,
                data: vec![],
            }, // RETURN
        ];

        let functions = analyze_functions(&instructions);

        assert_eq!(functions.len(), 1);
        assert_eq!(functions[0].selector, [0xa9, 0x05, 0x9c, 0xbb]);
        assert_eq!(functions[0].start, 0x234);
        assert_eq!(functions[0].end, 0x236);
    }
}
