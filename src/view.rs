use std::fmt;

use crate::{loader::Program, loader::SectionKind, parser::Instruction};

pub struct View {
    pub lines: Vec<Line>,
}

pub struct Line {
    pub offset: usize,
    pub kind: LineKind,
    pub comment: Option<String>,
}

pub enum LineKind {
    Label(String), // for ".runtime", "label_0011:", etc.
    Instruction(Instruction),
    HexDump(Vec<u8>), // for metadata or unknown blobs
    Blank,            // spacing
}

impl View {
    pub fn from_program(program: &Program) -> Self {
        let mut lines = Vec::new();

        for section in &program.sections {
            match section.kind {
                SectionKind::Init => {
                    lines.push(Line {
                        offset: section.start_pc,
                        kind: LineKind::Label(".init".to_string()),
                        comment: None,
                    });
                }
                SectionKind::Runtime => {
                    lines.push(Line {
                        offset: section.start_pc,
                        kind: LineKind::Label(".runtime".to_string()),
                        comment: None,
                    });
                }
                SectionKind::Metadata => {
                    lines.push(Line {
                        offset: section.start_pc,
                        kind: LineKind::Label(".metadata".to_string()),
                        comment: None,
                    });
                }
                SectionKind::Raw => {} // fallthrough
            }

            if let Some(instructions) = &section.instructions {
                for instruction in instructions {
                    lines.push(Line {
                        offset: section.start_pc + instruction.offset,
                        kind: LineKind::Instruction(instruction.clone()),
                        comment: None,
                    });
                }
            } else {
                lines.push(Line {
                    offset: 0,
                    kind: LineKind::HexDump(section.raw_bytes.to_vec()),
                    comment: None,
                });
            }
        }

        Self { lines }
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let LineKind::Label(name) = &self.kind {
            return write!(f, "{}:", name);
        }

        write!(f, "{:04x}: ", self.offset)?;

        let content = match &self.kind {
            LineKind::Instruction(instruction) => format!("{}", instruction),
            LineKind::HexDump(bytes) => hex::encode(bytes),
            LineKind::Blank => String::new(),
            LineKind::Label(_) => unreachable!(),
        };

        write!(f, "{:<35}", content)?;

        if let Some(comment) = &self.comment {
            write!(f, "; {}", comment)?;
        }

        Ok(())
    }
}

impl fmt::Display for View {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.lines {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

// fn decode_push_value(data: &[u8]) -> String {
//     if data.is_empty() {
//         return "0".to_string();
//     }

//     let mut value: u128 = 0;

//     // up to 16 bytes (u128)
//     if data.len() < 16 {
//         for &byte in data {
//             value = value * 256 + byte as u128;
//         }
//         format!("{}", value)
//     } else {
//         return "too large value".to_string();
//     }
// }

// fn is_push_instruction(opcode: u8) -> bool {
//     opcode >= 0x60 && opcode <= 0x7F
// }
