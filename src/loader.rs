use crate::parser::{self, Instruction};

pub struct Program {
    pub sections: Vec<Section>,
}

#[derive(Clone)]
pub struct Section {
    kind: SectionKind,
    raw_bytes: Vec<u8>,
    pub instructions: Option<Vec<Instruction>>,
    start_pc: usize,
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
            let instructions = parser::parse_bytecode(bytes);
            return Program {
                sections: vec![Section {
                    kind: SectionKind::Raw,
                    instructions: Some(instructions),
                    raw_bytes: bytes.to_vec(),
                    start_pc: 0,
                }],
            };
        }

        let (code_bytes, metadata_bytes) = Self::split_metadata(&bytes);

        let mut sections = Vec::new();

        let split_offset = if runtime {
            0 // no init in runtime mode, we assume runetime starts at 0 offset
        } else {
            Self::detect_runtime_split(code_bytes)
        };

        if split_offset > 0 {
            let init_bytes = &code_bytes[0..split_offset];
            sections.push(Section {
                kind: SectionKind::Init,
                instructions: Some(parser::parse_bytecode(init_bytes)),
                raw_bytes: init_bytes.to_vec(),
                start_pc: 0,
            });
        }

        let runtime_bytes = &code_bytes[split_offset..];
        if !runtime_bytes.is_empty() {
            sections.push(Section {
                kind: SectionKind::Runtime,
                instructions: Some(parser::parse_bytecode(runtime_bytes)),
                raw_bytes: runtime_bytes.to_vec(),
                start_pc: 0,
            });
        }

        if let Some(metadata) = metadata_bytes {
            sections.push(Section {
                kind: SectionKind::Metadata,
                instructions: None,
                raw_bytes: metadata.to_vec(),
                start_pc: 0,
            });
        }

        Program { sections }
    }

    fn detect_runtime_split(_: &[u8]) -> usize {
        // TODO: detect how to distinguish end of init section
        0
    }

    fn split_metadata(bytes: &[u8]) -> (&[u8], Option<&[u8]>) {
        // TODO: lookup end of the bytescode for metada length info
        (bytes, None)
    }
}
