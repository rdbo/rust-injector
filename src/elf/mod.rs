use std::os::unix::fs::FileExt;
use std::fs::File;
use std::mem::{size_of, transmute};

pub mod elfdefs;
use elfdefs::*;

macro_rules! separator2 {
    () => {
        println!("\t--------------------");
    }
}

pub fn read_ehdr(file : &File) -> Result<ElfW<Elf32_Ehdr, Elf64_Ehdr>, &'static str> {
    let mut magic : [u8;SELFMAG] = [0;SELFMAG];
    match file.read_exact_at(&mut magic, 0) {
        Err(_) => return Err("Unable to read ELF file"),
        _ => {  }
    }

    if magic != ELFMAG {
        return Err("The file is not ELF");
    }

    let mut class : [u8;1] = [ELFCLASSNONE];
    match file.read_exact_at(&mut class, EI_CLASS as u64) {
        Err(_) => return Err("Unable to read ELF class"),
        _ => {  }
    }

    if class[0] == ELFCLASS32 {
        let mut ehdrbuf : [u8;size_of::<Elf32_Ehdr>()] = [0;size_of::<Elf32_Ehdr>()];
        match file.read_exact_at(&mut ehdrbuf, 0) {
            Err(_) => return Err("Unable to read ELF header"),
            _ => {  }
        }

        let ehdr = unsafe {
            transmute::<[u8;size_of::<Elf32_Ehdr>()], Elf32_Ehdr>(ehdrbuf)
        };
        return Ok(ElfW::Elf32(ehdr));
    } else if class[0] == ELFCLASS64 {
        let mut ehdrbuf : [u8;size_of::<Elf64_Ehdr>()] = [0;size_of::<Elf64_Ehdr>()];
        match file.read_exact_at(&mut ehdrbuf, 0) {
            Err(_) => return Err("Unable to read ELF header"),
            _ => {  }
        }

        let ehdr = unsafe {
            transmute::<[u8;size_of::<Elf64_Ehdr>()], Elf64_Ehdr>(ehdrbuf)
        };
        return Ok(ElfW::Elf64(ehdr));
    } else {
        return Err("Invalid ELF class");
    }
}

pub fn enum_symbols<F>(file : &File, mut callback : F) -> Result<(), &'static str>
where F : FnMut(String, usize) {
    let ehdr = match read_ehdr(file) {
        Ok(hdr) => hdr,
        Err(e) => return Err(e)
    };

    println!("ELF Section: ");
    let result = ehdr.enum_sections(file, |name : String, offset : u64, entsize : u64, size : u64| -> bool {
        println!("\tName: {}", name);
        println!("\tOffset: {:#x}", offset);
        println!("\tEntsize: {:#x}", entsize);
        println!("\tSize: {:#x}", size);
        separator2!();
        return true;
    });

    match result {
        None => return Err("Could not enumerate ELF sections"),
        Some(_) => {  }
    }

    return Ok(());
}
