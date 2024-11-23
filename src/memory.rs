struct MemoryManager {
    memory: Vec<Option<Vec<Option<Vec<u8>>>>>,
}

impl MemoryManager {
    pub fn new() -> Self {
        Self {
            memory: vec![None; 0x3ff],
        }
    }

    pub fn add_page(&mut self, addr: u32) -> bool {
        let i = MemoryManager::get_first_entry_id(addr);
        let j = MemoryManager::get_second_entry_id(addr);

        if self.memory[i as usize].is_none() {
            let entry = vec![None; 0x3ff];
            self.memory[i as usize] = Some(entry);
        }
        let second_entry = self.memory[i as usize].as_mut().unwrap();
        if second_entry[j as usize].is_none() {
            let page = vec![0; 0xfff];
            second_entry[j as usize] = Some(page);
            true
        } else {
            println!(
                "Addr {:#x} already exists and do not need an addPage()!",
                addr
            );
            false
        }
    }

    pub fn get_byte(self, addr: u32) -> u8 {
        if !self.is_addr_exist(addr) {
            println!("Byte read to invalid addr {:#x}!", addr);
            return 0;
        }

        let i = MemoryManager::get_first_entry_id(addr);
        let j = MemoryManager::get_second_entry_id(addr);
        let k = MemoryManager::get_page_offset(addr);

        let second_entry = self.memory[i as usize].as_ref().unwrap();
        let page = second_entry[j as usize].as_ref().unwrap();
        page[k as usize]
    }

    pub fn set_byte(&mut self, addr: u32, val: u8) -> bool {
        if !self.is_addr_exist(addr) {
            println!("Byte write to invalid addr {:#x}!", addr);
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

    fn is_page_exist(&self, addr: u32) -> bool {
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
        manage.set_byte(addr, 111);
       println!("get byte: {}", manage.get_byte(addr)) ;
    }
}
