pub struct DataMemory {
    mem: Box<[u32]>
}


impl DataMemory {
    pub fn new(nwords: usize) -> DataMemory {
        DataMemory {
            mem: vec![0u32; nwords].into_boxed_slice()
        }
    }

    pub fn read(&self, addr: usize, size: usize) -> u32 {
        // Split byte address into word address and byte offset
        let word_addr = addr >> 2;
        let byte_offset = addr & 0x3;
        let byte_offset_in_bits = 8 * byte_offset;

        if word_addr >= self.mem.len() {
            panic!("Address 0x{:0x} out of range", word_addr);
        }

        if byte_offset + size > 4 {
            panic!("Unaligned memory access");
        }

        let word = self.mem[word_addr] >> byte_offset_in_bits;

        return match size {
            1 => word & 0xFF,   // isolate least significant byte
            2 => word & 0xFFFF, // isolate least significant halfword
            4 => word,
            _ => panic!("Can only read 1, 2, or 4 bytes at a time")
        };
    }

    pub fn write(&mut self, addr: usize, size: usize, data: u32) {
        // Split byte address into word address and byte offset
        let word_addr = addr >> 2;
        let byte_offset = addr & 0x3;
        let byte_offset_in_bits = 8 * byte_offset;

        if word_addr >= self.mem.len() {
            panic!("Address 0x{:0x} out of range", word_addr);
        }

        if byte_offset + size > 4 {
            panic!("Unaligned memory access");
        }

        let current_word = self.mem[word_addr];
        let mask = match size {
            1 => 0xFF,
            2 => 0xFFFF,
            4 => 0xFFFFFFFF,
            _ => panic!("Can only write 1, 2, or 4 bytes at a time")
        };
        let mask = mask << byte_offset_in_bits;
        let masked_current_word = current_word & !mask;
        let new_word = (data << byte_offset_in_bits) | masked_current_word;

        // Write back
        self.mem[word_addr] = new_word;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn common_constructor() {
        DataMemory::new(1024);
    }

    #[test]
    fn byte1() {
        let mut mem = DataMemory::new(2);
        let mut addr = 0x0;
        let mut size = 1;

        assert_eq!(mem.read(addr, size), 0);
        mem.write(addr, size, 0xFF);
        addr = 0x0;
        size = 4;
        assert_eq!(mem.read(addr, size), 0xFF);
    }

    #[test]
    fn byte2() {
        let mut mem = DataMemory::new(2);
        let mut addr = 0x1;
        let mut size = 1;

        assert_eq!(mem.read(addr, size), 0);
        mem.write(addr, size, 0xFF);
        addr = 0x0;
        size = 4;
        assert_eq!(mem.read(addr, size), 0xFF00);
    }

    #[test]
    fn byte3() {
        let mut mem = DataMemory::new(2);
        let mut addr = 0x2;
        let mut size = 1;

        assert_eq!(mem.read(addr, size), 0);
        mem.write(addr, size, 0xFF);
        addr = 0x0;
        size = 4;
        assert_eq!(mem.read(addr, size), 0xFF0000);
    }

    #[test]
    fn byte4() {
        let mut mem = DataMemory::new(2);
        let mut addr = 0x3;
        let mut size = 1;

        assert_eq!(mem.read(addr, size), 0);
        mem.write(addr, size, 0xFF);
        addr = 0x0;
        size = 4;
        assert_eq!(mem.read(addr, size), 0xFF000000);
    }

    #[test]
    fn lower_halfword() {
        let mut mem = DataMemory::new(2);
        let addr = 0x4;
        let size = 2;

        assert_eq!(mem.read(addr, size), 0);
        mem.write(addr, size, 0xF0F0);
        assert_eq!(mem.read(addr, size), 0xF0F0);
    }

    #[test]
    fn upper_halfword() {
        let mut mem = DataMemory::new(2);
        let addr = 0x6;
        let size = 2;

        assert_eq!(mem.read(addr, size), 0);
        mem.write(addr, size, 0xF0F0);
        assert_eq!(mem.read(addr, size), 0xF0F0);
    }

    #[test]
    fn full_word() {
        let mut mem = DataMemory::new(2);
        let addr = 0x4;
        let size = 4;

        assert_eq!(mem.read(addr, size), 0);
        mem.write(addr, size, 0xF0F0F0F0);
        assert_eq!(mem.read(addr, size), 0xF0F0F0F0);
    }

    #[test]
    #[should_panic]
    fn unaligned_halfword() {
        let mem = DataMemory::new(2);
        let addr = 0x3;
        let size = 2;
        // Attempt to read addrs 0x3 and 0x4, which crosses a word boundary
        mem.read(addr, size);
    }

    #[test]
    #[should_panic]
    fn unaligned_word() {
        let mem = DataMemory::new(2);
        let addr = 0x2;
        let size = 4;
        // Attempt to read addrs 0x2 through 0x5, which crosses a word boundary
        mem.read(addr, size);
    }

    #[test]
    #[should_panic]
    fn read_outside_range() {
        // Create a 2-word memory space with valid addresses 0x0 through 0x7
        let mem = DataMemory::new(2);
        // Try to read memory address 0x8
        let addr = 0x8;
        let size = 1;
        mem.read(addr, size);
    }

    #[test]
    #[should_panic]
    fn write_outside_range() {
        // Create a 2-word memory space with valid addresses 0x0 through 0x7
        let mut mem = DataMemory::new(2);
        // Try to write to memory address 0x8
        let addr = 0x8;
        let size = 1;
        mem.write(addr, size, 0x1);
    }

}
