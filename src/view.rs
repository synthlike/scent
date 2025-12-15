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
    pub fn from_program(program: &Program, decorated: bool) -> Self {
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
                    let mut comment = None;

                    if decorated && !instruction.data.is_empty() {
                        comment = decorate_push_data(&instruction.data)
                    }

                    lines.push(Line {
                        offset: section.start_pc + instruction.offset,
                        kind: LineKind::Instruction(instruction.clone()),
                        comment,
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

fn decorate_push_data(data: &[u8]) -> Option<String> {
    if data.is_empty() {
        return None;
    }

    // don't interpret single byte
    if data.len() == 1 {
        return None;
    }

    // if data.len() == 4 {
    //  TODO: function selectors
    // }

    // if data starts with 0x00/0x01 it's likely a number, zeros imply padding.
    if data[0] == 0x00 || data[0] == 0x01 {
        // unless it's too big, then it's probably an address?
        if data.len() < 16 {
            let val = bytes_to_u128(data);
            return Some(format!("{}", val));
        }
    }

    // if data is longer than two bytes and is purely printable characters we assume it's string
    if data.len() > 2 && data.iter().all(|&b| b >= 0x20 && b <= 0x7e) {
        let text = str::from_utf8(data).unwrap_or("");
        return Some(format!("{:?}", text));
    }

    return None;
}

fn bytes_to_u128(bytes: &[u8]) -> u128 {
    let mut val = 0u128;
    for &b in bytes {
        val = (val << 8) | b as u128;
    }
    val
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

        write!(f, "{:<20}", content)?;

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
