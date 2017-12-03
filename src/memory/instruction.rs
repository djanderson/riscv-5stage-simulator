//! Read-only instruction memory.
//!
//! Provides a loader for disassembler output.


use regex::{Captures, Regex};

use consts::HALT;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::num::ParseIntError;


/// Regex to parse disassembler output
const RE: &str = r"(?x)
    ^[[:blank:]]+
    (?P<addr>[[:xdigit:]]+):    # Captures ... (addr): xx xx xx xx ...
    [[:blank:]]+
    (?P<byte1>[[:xdigit:]]{2})  # Captures ... addr: (xx) xx xx xx ...
    [[:blank:]]
    (?P<byte2>[[:xdigit:]]{2})  # Captures ... addr: xx (xx) xx xx ...
    [[:blank:]]
    (?P<byte3>[[:xdigit:]]{2})  # Captures ... addr: xx xx (xx) xx ...
    [[:blank:]]
    (?P<byte4>[[:xdigit:]]{2})  # Captures ... addr: xx xx xx (xx) ...
    .*$
";


/// A read-only instruction memory trait.
pub trait InstructionMemory {
    fn read(&self, addr: usize) -> u32;
}


/// Instruction memory that populates memory from disassembler output.
pub struct DisassemblyInstructionMemory {
    mem: Vec<u32>,
}


impl DisassemblyInstructionMemory {
    /// Constructs a new `DisassemblyInstructionMemory`.
    ///
    /// `disassembly` must be an open text file containing lines of the form:
    ///
    /// ```text
    ///      16c:	00 15 05 13    addi x10 , x10 , 1
    /// ```
    ///
    /// The first such matching line must have address 0.
    /// Non-matching lines are ignored.
    ///
    pub fn new(disassembly: &File) -> DisassemblyInstructionMemory {
        let file = BufReader::new(disassembly);
        let mut mem = Vec::new();
        let regex = Regex::new(RE).unwrap();

        // Load each line of disassembly into memory
        for line in file.lines() {
            let l = line.expect("failed to read line");
            match regex.captures(&l) {
                Some(caps) => {
                    let addr = extract_addr(&caps).unwrap();
                    // Test that addr matches the actual location in memory
                    assert_eq!(addr, (mem.len() * 4) as u32);
                    let insn = extract_insn(&caps).unwrap();
                    mem.push(insn);
                }
                // Ignore lines that don't match the regex
                None => {}
            }
        }

        mem.push(HALT);

        DisassemblyInstructionMemory { mem }
    }
}


impl InstructionMemory for DisassemblyInstructionMemory {
    /// Reads an instruction from `InstructionMemory`.
    ///
    /// The requested address is right-shifted by 2 to ensure word alignment.
    ///
    fn read(&self, addr: usize) -> u32 {
        let word_addr = addr >> 2;
        let byte_offset = addr & 0x3;

        if word_addr >= self.mem.len() {
            panic!("Address {:#0x} out of range", addr);
        }

        if byte_offset != 0 {
            panic!("Unaligned memory access at {:#0x}", addr);
        }

        self.mem[word_addr]
    }
}


pub struct TestInstructionMemory {
    mem: Vec<u32>,
}


impl TestInstructionMemory {
    pub fn new(mem: Vec<u32>) -> TestInstructionMemory {
        TestInstructionMemory { mem }
    }
}


impl InstructionMemory for TestInstructionMemory {
    /// Reads an instruction from `InstructionMemory`.
    ///
    /// The requested address is right-shifted by 2 to ensure word alignment.
    ///
    fn read(&self, addr: usize) -> u32 {
        let word_addr = addr >> 2;
        let byte_offset = addr & 0x3;

        if word_addr >= self.mem.len() {
            panic!("Address {:#0x} out of range", addr);
        }

        if byte_offset != 0 {
            panic!("Unaligned memory access at {:#0x}", addr);
        }

        self.mem[word_addr]
    }
}


/// Extracts regex captures related to the memory address and converts to u32.
fn extract_addr(caps: &Captures) -> Result<u32, ParseIntError> {
    let s = caps.name("addr").unwrap().as_str();
    u32::from_str_radix(&s, 16)
}


/// Extracts regex captures related to the instruction and converts to u32.
fn extract_insn(caps: &Captures) -> Result<u32, ParseIntError> {
    let s: String = caps.name("byte1").unwrap().as_str().to_owned() +
        &caps.name("byte2").unwrap().as_str().to_owned() +
        &caps.name("byte3").unwrap().as_str().to_owned() +
        &caps.name("byte4").unwrap().as_str().to_owned();

    u32::from_str_radix(&s, 16)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regex() {
        let haystack: &str = "     16c:	00 15 05 13    addi x10 , x10 , 1";
        let needle = Regex::new(RE).unwrap();
        let caps = needle.captures(haystack).unwrap();
        assert_eq!("16c", &caps["addr"]);
        assert_eq!("00", &caps["byte1"]);
        assert_eq!("15", &caps["byte2"]);
        assert_eq!("05", &caps["byte3"]);
        assert_eq!("13", &caps["byte4"]);

        let haystack: &str = "1c4 <FAIL____src_ins_assembly_test_s>:";
        assert!(needle.captures(haystack).is_none());
    }

    #[test]
    fn extract_addr_from_regex_captures() {
        let haystack: &str = "     16c:	00 15 05 13    addi x10 , x10 , 1";
        let needle = Regex::new(RE).unwrap();
        let caps = needle.captures(haystack).unwrap();
        let addr = extract_addr(&caps).unwrap();
        assert_eq!(addr, 0x16c);
    }

    #[test]
    fn extract_insn_from_regex_captures() {
        let haystack: &str = "     16c:	00 15 05 13    addi x10 , x10 , 1";
        let needle = Regex::new(RE).unwrap();
        let caps = needle.captures(haystack).unwrap();
        let insn = extract_insn(&caps).unwrap();
        assert_eq!(insn, 0x00_15_05_13);
    }

}
