use crate::{
    analysis::{Analysis, FunctionEntrypoint},
    parser::{self, Instruction},
};

pub struct Program {
    pub sections: Vec<Section>,
    pub entrypoints: Vec<FunctionEntrypoint>,
}

#[derive(Clone)]
pub struct Section {
    pub kind: SectionKind,
    pub raw_bytes: Vec<u8>,
    pub instructions: Option<Vec<Instruction>>,
    pub start_pc: usize,
}

#[derive(Clone)]
pub enum SectionKind {
    Init,
    Runtime,
    Metadata,
    Raw,
}

impl Program {
    pub fn load(bytes: &[u8], raw: bool, runtime: bool) -> Self {
        if raw {
            return Program {
                sections: vec![Section {
                    kind: SectionKind::Raw,
                    instructions: Some(parser::parse_bytecode(bytes)),
                    raw_bytes: bytes.to_vec(),
                    start_pc: 0,
                }],
                entrypoints: Vec::new(),
            };
        }

        let mut sections = Vec::new();

        let metadata_split_offset = Self::detect_metadata_split(&bytes);

        let code_bytes = &bytes[0..metadata_split_offset];

        let runtime_split_offset = if runtime {
            0 // no init in runtime mode, we assume runtime starts at 0 offset
        } else {
            Self::detect_runtime_split(&code_bytes)
        };

        if runtime_split_offset > 0 {
            let init_bytes = &code_bytes[0..runtime_split_offset];
            sections.push(Section {
                kind: SectionKind::Init,
                instructions: Some(parser::parse_bytecode(init_bytes)),
                raw_bytes: init_bytes.to_vec(),
                start_pc: 0,
            });
        }

        let mut entrypoints = Vec::new();

        let runtime_bytes = &code_bytes[runtime_split_offset..];
        if !runtime_bytes.is_empty() {
            let instructions = parser::parse_bytecode(runtime_bytes);

            sections.push(Section {
                kind: SectionKind::Runtime,
                instructions: Some(instructions.clone()),
                raw_bytes: runtime_bytes.to_vec(),
                start_pc: 0,
            });

            let analysis = Analysis::from_instructions(&instructions);
            entrypoints = analysis.function_entrypoints
        }

        if metadata_split_offset < bytes.len() {
            sections.push(Section {
                kind: SectionKind::Metadata,
                instructions: None,
                raw_bytes: bytes[metadata_split_offset..bytes.len()].to_vec(),
                start_pc: 0,
            });
        }

        Program {
            sections,
            entrypoints,
        }
    }

    // Detect runtime starts by looking for 0xF3FE bytes.
    // It looks like solidity uses that as delimeter, although might break.
    fn detect_runtime_split(bytes: &[u8]) -> usize {
        for i in 0..bytes.len().saturating_sub(1) {
            if bytes[i] == 0xF3 && bytes[i + 1] == 0xFE {
                return i + 2;
            }
        }

        0
    }

    // Detect metadata starts by looking at the last two bytes - potential length of metadata.
    // Then, based on the potential length it looks for pair of magic bytes 0xa2 + 0x64/0x65/0x66.
    // When metadata not found it returns the size of the whole bytecode.
    fn detect_metadata_split(bytes: &[u8]) -> usize {
        if bytes.len() < 2 {
            return bytes.len();
        }

        let metadata_length =
            u16::from_be_bytes([bytes[bytes.len() - 2], bytes[bytes.len() - 1]]) as usize;

        let metadata_start = bytes.len() - metadata_length - 2; // 2 is the size of metadata length

        if metadata_start + 1 < bytes.len() {
            let first = bytes[metadata_start];
            let second = bytes[metadata_start + 1];
            if first == 0xa2 && (second >= 0x64 && second <= 0x66) {
                return metadata_start;
            }
        }

        bytes.len()
    }
}
