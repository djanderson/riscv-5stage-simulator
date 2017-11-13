use instruction::GUARD_INSTRUCTION;

use regex::{Captures, Regex};

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;


const RE: &str = r"(?x)
    ^[[:blank:]]+
    (?P<addr>[[:xdigit:]]+)
    :
    [[:blank:]]+
    (?P<byte1>[[:xdigit:]]{2})
    [[:blank:]]
    (?P<byte2>[[:xdigit:]]{2})
    [[:blank:]]
    (?P<byte3>[[:xdigit:]]{2})
    [[:blank:]]
    (?P<byte4>[[:xdigit:]]{2})
    .*$
";


pub struct InstructionMemory {
    pub mem: Vec<u32>,
}


impl InstructionMemory {
    pub fn new(f: &File) -> InstructionMemory {
        let file = BufReader::new(f);
        let mut mem = Vec::new();
        let regex = Regex::new(RE).unwrap();

        for line in file.lines() {
            let l = match line {
                Ok(v) => v,
                Err(e) => panic!("Couldn't read line: {:?}", e),
            };
            match regex.captures(&l) {
                Some(caps) => {
                    let addr = extract_addr(&caps);
                    let insn = extract_insn(&caps);
                    mem.push(insn);
                    assert_eq!(addr, ((mem.len() - 1) * 4) as u32);
                }
                None => {}
            }
        }

        mem.push(GUARD_INSTRUCTION);

        InstructionMemory { mem }
    }

    pub fn read(&self, addr: usize) -> u32 {
        let word_addr = addr >> 2;

        if word_addr >= self.mem.len() {
            panic!("Address 0x{:0x} out of range", word_addr);
        }

        self.mem[word_addr]
    }
}


fn extract_addr(caps: &Captures) -> u32 {
    let s = caps.name("addr").unwrap().as_str();
    let addr = u32::from_str_radix(&s, 16).unwrap();

    addr
}


fn extract_insn(caps: &Captures) -> u32 {
    let s: String = caps.name("byte1").unwrap().as_str().to_owned() +
        &caps.name("byte2").unwrap().as_str().to_owned() +
        &caps.name("byte3").unwrap().as_str().to_owned() +
        &caps.name("byte4").unwrap().as_str().to_owned();

    let insn = u32::from_str_radix(&s, 16).unwrap();

    insn
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
        let addr = extract_addr(&caps);
        assert_eq!(addr, 0x16c);
    }

    #[test]
    fn extract_insn_from_regex_captures() {
        let haystack: &str = "     16c:	00 15 05 13    addi x10 , x10 , 1";
        let needle = Regex::new(RE).unwrap();
        let caps = needle.captures(haystack).unwrap();
        let insn = extract_insn(&caps);
        assert_eq!(insn, 0x00_15_05_13);
    }

}
