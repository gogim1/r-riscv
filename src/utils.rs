use std::fmt;

use elf::{
    abi::EM_RISCV,
    endian::{AnyEndian, EndianParse},
    file::FileHeader,
    ElfBytes,
};

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
