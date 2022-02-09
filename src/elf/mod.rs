use std::os::unix::fs::FileExt;
use std::fs::File;
use std::mem::{size_of, transmute};
use std::io::{Seek, SeekFrom, BufRead, BufReader};

pub mod elfdefs;
use elfdefs::*;

macro_rules! separator {
    () => {
        println!("====================");
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
where F : FnMut(&ElfW<Elf32_Ehdr, Elf64_Ehdr>, String, u64) -> bool {
    let ehdr = match read_ehdr(file) {
        Ok(hdr) => hdr,
        Err(e) => return Err(e)
    };

    let mut strtab_off : Option<u64> = None;
    let mut symtab_off : Option<u64> = None;
    let mut symtab_entsize : u64 = 0;
    let mut symtab_size : u64 = 0;

    ehdr.enum_sections(file, |name : String, offset : u64, entsize : u64, size : u64| -> bool {
        if strtab_off != None && symtab_off != None {
            return false;
        }

        if name == ".symtab" {
            println!("Symbol Table: ");
            println!("\tName: {}", name);
            println!("\tOffset: {:#x}", offset);
            println!("\tEntsize: {:#x}", entsize);
            println!("\tSize: {:#x}", size);
            separator!();

            symtab_off = Some(offset);
            symtab_entsize = entsize;
            symtab_size = size;
        } else if name == ".strtab" {
            println!("String Table: ");
            println!("\tName: {}", name);
            println!("\tOffset: {:#x}", offset);
            println!("\tEntsize: {:#x}", entsize);
            println!("\tSize: {:#x}", size);
            separator!();
            strtab_off = Some(offset);
        }

        return true;
    });

    let strtab_off = match strtab_off {
        Some(off) => off,
        None => return Err("Unable to retrieve string table from ELF file")
    };
    let symtab_off = match symtab_off {
        Some(off) => off,
        None => return Err("Unable to retrieve symbol table from ELF file")
    };
    let entnum = symtab_size / symtab_entsize;
    for i in 0..entnum {
        match ehdr.get_class() {
            ELFCLASS32 => {
                let mut symbuf : [u8;size_of::<Elf32_Sym>()] = [0;size_of::<Elf32_Sym>()];
                file.read_exact_at(&mut symbuf, symtab_off + i * symtab_entsize);
                let sym : Elf32_Sym = unsafe {
                    transmute::<[u8;size_of::<Elf32_Sym>()], Elf32_Sym>(symbuf)
                };

                let mut symstr_buf : Vec<u8> = vec![];
                let mut reader = BufReader::new(file);
                reader.seek(SeekFrom::Start(strtab_off + sym.st_name as u64));
                reader.read_until(b'\x00', &mut symstr_buf);
                symstr_buf.pop(); // remove null terminator

                let symstr = String::from_utf8_lossy(&symstr_buf).to_string();
                let symval = sym.st_value as u64;

                if !callback(&ehdr, symstr, symval) {
                    break;
                }
            }

            ELFCLASS64 => {
                let mut symbuf : [u8;size_of::<Elf64_Sym>()] = [0;size_of::<Elf64_Sym>()];
                file.read_exact_at(&mut symbuf, symtab_off + i * symtab_entsize);
                let sym : Elf64_Sym = unsafe {
                    transmute::<[u8;size_of::<Elf64_Sym>()], Elf64_Sym>(symbuf)
                };

                let mut symstr_buf : Vec<u8> = vec![];
                let mut reader = BufReader::new(file);
                reader.seek(SeekFrom::Start(strtab_off + sym.st_name as u64));
                reader.read_until(b'\x00', &mut symstr_buf);
                symstr_buf.pop(); // remove null terminator

                let symstr = String::from_utf8_lossy(&symstr_buf).to_string();
                let symval = sym.st_value as u64;

                if !callback(&ehdr, symstr, symval) {
                    break;
                }
            }

            _ => break
        }
    }

    return Ok(());
}
