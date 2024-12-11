use std::fmt;

use elf::{
    abi::EM_RISCV,
    endian::{AnyEndian, EndianParse},
    file::FileHeader,
    ElfBytes,
};

use super::memory::MemoryManager;

pub fn print_elf_header<E>(hdr: &FileHeader<E>)
where
    E: EndianParse + fmt::Debug,
{
    println!("Type: {:?}", hdr.class);

    let endianness = if hdr.endianness.is_little() {
        "Little Endian"
    } else {
        "Large Endian"
    };
    println!("Encoding: {endianness}");

    if hdr.e_machine == EM_RISCV {
        println!("ISA: RISC-V({:#02x})", hdr.e_machine);
    } else {
        panic!("ISA: Unsupported({:#02x})", hdr.e_machine);
    }
}

pub fn print_elf_info(file: &ElfBytes<'_, AnyEndian>) {
    println!("==========ELF Information==========");
    print_elf_header(&file.ehdr);

    let (shdrs_opt, strtab_opt) = file
        .section_headers_with_strtab()
        .expect("shdrs offsets should be valid");
    let (shdrs, strtab) = (
        shdrs_opt.expect("Should have shdrs"),
        strtab_opt.expect("Should have strtab"),
    );

    println!("Number of Sections: {}", shdrs.len());
    println!("ID\tName\t\tAddress\tSize");
    for (idx, hdr) in shdrs.iter().enumerate() {
        let name = strtab
            .get(hdr.sh_name as usize)
            .expect("Failed to get section name");
        let address = hdr.sh_addr;
        let size = hdr.sh_size;
        println!("[{}]\t{: <12}\t{:#x}\t{}", idx, name, address, size);
    }

    let segments = file.segments().expect("shdrs offsets should be valid");
    println!("Number of Segments: {}", segments.len());
    println!("ID\tFlags\tAddress\tFSize\tMSize");
    for (idx, seg) in segments.iter().enumerate() {
        println!(
            "[{}]\t{:#x}\t{:#x}\t{}\t{}",
            idx, seg.p_flags, seg.p_vaddr, seg.p_filesz, seg.p_memsz
        );
    }

    println!("===================================");
}

pub fn load_elf_to_memory(file: &ElfBytes<'_, AnyEndian>, memory: &mut MemoryManager) {
    let segments = file.segments().unwrap();
    for (i, seg) in segments.iter().enumerate() {
        let addr = seg.p_vaddr;
        let filesz = seg.p_filesz;
        let memsz = seg.p_memsz;
        if addr + memsz > 0xffffffff {
            panic!(
                "ELF address space larger than 32bit! Seg {} has max addr of {:#x}",
                i,
                addr + memsz
            );
        }
        for p in addr..(addr + memsz) {
            let p = p as u32;
            if !memory.is_page_exist(p) {
                memory.add_page(p);
            }
            if p < (addr + filesz) as u32 {
                let val = file.segment_data(&seg).unwrap()[(p - addr as u32) as usize];
                memory.set_byte(p, val);
            } else {
                memory.set_byte(p, 0);
            }
        }
    }
}

pub fn get_entry<E>(hdr: &FileHeader<E>) -> u32
where
    E: EndianParse + fmt::Debug,
{
    hdr.e_entry as u32
}

#[cfg(test)]
mod tests {
    use std::fs;

    use elf::{endian::AnyEndian, ElfBytes};

    use crate::*;

    #[test]
    fn read_elf() {
        let path = std::path::PathBuf::from("riscv-elf/ackermann.riscv");
        let file_data = std::fs::read(path).unwrap();
        let file = ElfBytes::<AnyEndian>::minimal_parse(&file_data).unwrap();

        utils::print_elf_info(&file);

        let mut memory = memory::MemoryManager::new();
        utils::load_elf_to_memory(&file, &mut memory);
        fs::write("/tmp/elf_memory", memory.dump_memory()).unwrap();
    }
}
