use std::collections::HashMap;
use strum::IntoEnumIterator;

use crate::args::Args;
use crate::memory::MemoryManager;
use crate::regs::{Reg, RegSet};

const STACK_BASE_ADDR: u32 = 0x80000000;
const STACK_SIZE: u32 = 0x400000;

pub struct Simulator<'a> {
    memory: &'a mut MemoryManager,
    args: &'a Args,
    pc: u32,
    regs: RegSet,
}

impl Simulator<'_> {
    pub fn new<'a>(args: &'a Args, memory: &'a mut MemoryManager, pc: u32) -> Simulator<'a> {
        let mut sim = Simulator {
            args,
            memory,
            pc,
            regs: HashMap::new(),
        };

        for reg in Reg::iter() {
            sim.regs.insert(reg, 0);
        }
        sim.regs.insert(Reg::RegSP, STACK_BASE_ADDR);

        for addr in STACK_BASE_ADDR - STACK_SIZE + 1..=STACK_BASE_ADDR {
            if !sim.memory.is_page_exist(addr) {
                sim.memory.add_page(addr);
            }
            sim.memory.set_byte(addr, 0);
        }
        sim
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
