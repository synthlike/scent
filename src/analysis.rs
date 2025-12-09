use std::fmt;

use crate::parser::Instruction;

pub struct Analysis {
    pub function_selectors: Vec<FunctionSelector>,
}

#[derive(PartialEq)]
pub struct FunctionSelector {
    pub offset: usize,
    pub selector: [u8; 4],
    pub name: Option<String>, // default or from sift selectors
}

impl fmt::Debug for FunctionSelector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Selector {{ offset: {}, selector: 0x{}, name: {} }}",
            self.offset,
            hex::encode(&self.selector),
            self.name.as_deref().unwrap_or("unnamed"),
        )
    }
}

pub fn analyze_function_selectors(instructions: &[Instruction]) -> Vec<FunctionSelector> {
    instructions
        .windows(2)
        .filter_map(|w| {
            let first = &w[0];
            let second = &w[1];

            // PUSH <selector>
            if first.opcode == 0x63 && first.data.len() == 4 &&
            // EQ
            second.opcode == 0x14
            {
                let mut selector = [0u8; 4];
                selector.copy_from_slice(&first.data);

                Some(FunctionSelector {
                    offset: first.offset,
                    selector,
                    name: Some(format!("func_{:08x}", u32::from_be_bytes(selector))),
                })
            } else {
                None
            }
        })
        .collect()
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
                name: Some("func_11223344".to_string()),
            }
        )
    }
}
