pub struct MemoryManager {
    memory: Vec<Option<Vec<Option<Vec<u8>>>>>,
}

impl MemoryManager {
    pub fn new() -> Self {
        Self {
            memory: vec![None; 0x400],
        }
    }

    pub fn add_page(&mut self, addr: u32) -> bool {
        let i = MemoryManager::get_first_entry_id(addr);
        let j = MemoryManager::get_second_entry_id(addr);

        if self.memory[i as usize].is_none() {
            let entry = vec![None; 0x400];
            self.memory[i as usize] = Some(entry);
        }
        let second_entry = self.memory[i as usize].as_mut().unwrap();
        if second_entry[j as usize].is_none() {
            let page = vec![0; 0x1000];
            second_entry[j as usize] = Some(page);
            true
        } else {
            println!(
                "Addr {:#010x} already exists and do not need an addPage()!",
                addr
            );
            false
        }
    }

    pub fn get_byte(&self, addr: u32) -> Option<u8> {
        if !self.is_addr_exist(addr) {
            println!("Byte read to invalid addr {:#010x}!", addr);
            return None;
        }

        let i = MemoryManager::get_first_entry_id(addr);
        let j = MemoryManager::get_second_entry_id(addr);
        let k = MemoryManager::get_page_offset(addr);

        let second_entry = self.memory[i as usize].as_ref().unwrap();
        let page = second_entry[j as usize].as_ref().unwrap();
        Some(page[k as usize])
    }

    pub fn get_short(&self, addr: u32) -> Option<u16> {
        let b1: u16 = self.get_byte(addr)?.into();
        let b2: u16 = self.get_byte(addr + 1)?.into();
        Some(b1 + (b2 << 8))
    }

    pub fn get_int(&self, addr: u32) -> Option<u32> {
        let b1: u32 = self.get_byte(addr)?.into();
        let b2: u32 = self.get_byte(addr + 1)?.into();
        let b3: u32 = self.get_byte(addr + 2)?.into();
        let b4: u32 = self.get_byte(addr + 3)?.into();
        Some(b1 + (b2 << 8) + (b3 << 16) + (b4 << 24))
    }

    pub fn get_long(&self, addr: u32) -> Option<u64> {
        let b1: u64 = self.get_byte(addr)?.into();
        let b2: u64 = self.get_byte(addr + 1)?.into();
        let b3: u64 = self.get_byte(addr + 2)?.into();
        let b4: u64 = self.get_byte(addr + 3)?.into();
        let b5: u64 = self.get_byte(addr + 4)?.into();
        let b6: u64 = self.get_byte(addr + 5)?.into();
        let b7: u64 = self.get_byte(addr + 6)?.into();
        let b8: u64 = self.get_byte(addr + 7)?.into();
        Some(
            b1 + (b2 << 8)
                + (b3 << 16)
                + (b4 << 24)
                + (b5 << 32)
                + (b6 << 40)
                + (b7 << 48)
                + (b8 << 56),
        )
    }

    pub fn set_byte(&mut self, addr: u32, val: u8) -> bool {
        if !self.is_addr_exist(addr) {
            println!("Byte write to invalid addr {:#010x}!", addr);
            return false;
        }

        let i = MemoryManager::get_first_entry_id(addr);
        let j = MemoryManager::get_second_entry_id(addr);
        let k = MemoryManager::get_page_offset(addr);

        self.memory[i as usize].as_mut().unwrap()[j as usize]
            .as_mut()
            .unwrap()[k as usize] = val;
        true
    }

    pub fn set_short(&mut self, addr: u32, val: u16) -> bool {
        if !self.is_addr_exist(addr) {
            println!("Byte write to invalid addr {:#010x}!", addr);
            return false;
        }

        self.set_byte(addr, (val & 0xff) as u8);
        self.set_byte(addr + 1, ((val >> 8) & 0xff) as u8);
        true
    }

    pub fn set_int(&mut self, addr: u32, val: u32) -> bool {
        if !self.is_addr_exist(addr) {
            println!("Byte write to invalid addr {:#010x}!", addr);
            return false;
        }

        self.set_byte(addr, (val & 0xff) as u8);
        self.set_byte(addr + 1, ((val >> 8) & 0xff) as u8);
        self.set_byte(addr + 2, ((val >> 16) & 0xff) as u8);
        self.set_byte(addr + 3, ((val >> 24) & 0xff) as u8);
        true
    }

    pub fn set_long(&mut self, addr: u32, val: u64) -> bool {
        if !self.is_addr_exist(addr) {
            println!("Byte write to invalid addr {:#010x}!", addr);
            return false;
        }

        self.set_byte(addr, (val & 0xff) as u8);
        self.set_byte(addr + 1, ((val >> 8) & 0xff) as u8);
        self.set_byte(addr + 2, ((val >> 16) & 0xff) as u8);
        self.set_byte(addr + 3, ((val >> 24) & 0xff) as u8);
        self.set_byte(addr + 4, ((val >> 32) & 0xff) as u8);
        self.set_byte(addr + 5, ((val >> 40) & 0xff) as u8);
        self.set_byte(addr + 6, ((val >> 48) & 0xff) as u8);
        self.set_byte(addr + 7, ((val >> 56) & 0xff) as u8);
        true
    }

    pub fn copy_from(&mut self, src: u32, dst: u32, len: usize) -> bool {
        for i in 0..len {
            if !self.is_addr_exist(src + i as u32) {
                println!("Data copy from invalid addr {:#010x}!", src);
                return false;
            }
            if !self.is_addr_exist(dst + i as u32) {
                println!("Data copy to invalid addr {:#010x}!", src);
                return false;
            }
            let val = self.get_byte(src + i as u32).unwrap();
            self.set_byte(dst + i as u32, val);
        }
        true
    }

    pub fn dump_memory(&self) -> String {
        let mut ret = String::new();
        ret.push_str("Memory Pages:\n");

        for (i, second_entry_opt) in self.memory.iter().enumerate() {
            let i = i as u32;
            if let Some(second_entry) = second_entry_opt {
                ret.push_str(&format!("{:#010x}-{:#010x}:\n", i << 22, (i + 1) << 22).to_owned());

                for (j, page_opt) in second_entry.iter().enumerate() {
                    let j = j as u32;
                    if let Some(page) = page_opt {
                        ret.push_str(
                            &format!(
                                "  {:#010x}-{:#010x}:\n",
                                i << 22 | j << 12,
                                i << 22 | (j + 1) << 12
                            )
                            .to_owned(),
                        );

                        for (k, val) in page.iter().enumerate() {
                            let k = k as u32;
                            ret.push_str(
                                &format!("   {:#010x}: {:#04x}\n", (i << 22) | (j << 12) | k, val)
                                    .to_owned(),
                            );
                        }
                    }
                }
            }
        }
        ret
    }

    pub fn print_info(&self) {
        println!("{}", self.dump_memory());
    }

    pub fn is_page_exist(&self, addr: u32) -> bool {
        self.is_addr_exist(addr)
    }

    fn is_addr_exist(&self, addr: u32) -> bool {
        let i = MemoryManager::get_first_entry_id(addr);
        let j = MemoryManager::get_second_entry_id(addr);

        let first_entry_opt = &self.memory[i as usize];
        if first_entry_opt.is_some() && first_entry_opt.as_ref().unwrap()[j as usize].is_some() {
            true
        } else {
            false
        }
    }

    fn get_first_entry_id(addr: u32) -> u32 {
        (addr >> 22) & 0x3ff
    }

    fn get_second_entry_id(addr: u32) -> u32 {
        (addr >> 12) & 0x3ff
    }

    fn get_page_offset(addr: u32) -> u32 {
        addr & 0xfff
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_and_get_byte() {
        let mut manage = MemoryManager::new();
        let addr = 1_u32;
        manage.add_page(addr);

        manage.set_byte(addr, 123);
        assert!(manage.get_byte(addr).unwrap() == 123);

        manage.set_short(addr, 123);
        assert!(manage.get_short(addr).unwrap() == 123);

        manage.set_int(addr, 123);
        assert!(manage.get_int(addr).unwrap() == 123);

        manage.set_long(addr, 123);
        assert!(manage.get_long(addr).unwrap() == 123);
    }

    #[test]
    fn page_miss() {
        let mut manage = MemoryManager::new();
        manage.dump_memory();
        let addr = 1_u32;

        assert!(!manage.set_byte(addr, 123));
        assert!(manage.get_short(addr).is_none());

        manage.add_page(addr);
        assert!(manage.set_byte(addr, 123));
        assert!(manage.get_short(addr).is_some());
    }

    #[test]
    fn copy_from() {
        let mut manage = MemoryManager::new();
        manage.add_page(0_u32);

        manage.set_byte(1_u32, 1);
        manage.set_byte(2_u32, 2);
        manage.set_byte(3_u32, 3);

        assert!(manage.copy_from(1, 4, 3));
        assert!(manage.get_byte(4).unwrap() == 1);
        assert!(manage.get_byte(5).unwrap() == 2);
        assert!(manage.get_byte(6).unwrap() == 3);
    }
}
