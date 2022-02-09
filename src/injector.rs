use regex::Regex;
use crate::proc::{enum_maps, MapsEntry};
use crate::elf::enum_symbols;
use crate::elf::elfdefs::{ElfW_Ehdr, ElfW, Elf32_Ehdr, Elf64_Ehdr, ET_EXEC};
use std::fs::File;

macro_rules! separator {
    () => {
        println!("====================");
    }
}

fn find_libc(pid : i32) -> Option<MapsEntry> {
    let mut libc_entry : Option<MapsEntry> = None;
    let re = Regex::new(r".*/(libc[.\-].*)").ok()?;
    enum_maps(pid, |entry : MapsEntry| {
        match re.captures(entry.path.as_str()) {
            None => return true,
            Some(_) => {  }
        };

        libc_entry = Some(entry);
        return false;
    });

    return libc_entry;
}

fn find_dlopen(libc_entry : &MapsEntry) -> Result<u64, String> {
    let libc_file = match File::open(&libc_entry.path) {
        Ok(f) => f,
        Err(e) => return Err(format!("Could not open libc file: {}", e))
    };
    
    let mut dlopen_addr : Option<u64> = None;
    let result = enum_symbols(&libc_file, |ehdr : &ElfW<Elf32_Ehdr, Elf64_Ehdr>, symbol : String, value : u64| -> bool {
        let mut value = value;
        if symbol != "__libc_dlopen_mode" {
            return true;
        }

        println!("Libc dlopen info: ");
        println!("\tSymbol: {}", symbol);
        if ehdr.get_type() != ET_EXEC {
            println!("\tAddress (rel): {:#x}", value);
            value += libc_entry.base as u64; // calculate absolute address
        }
        println!("\tAddress: {:#x}", value);
        dlopen_addr = Some(value);
        return false;
    });

    match result {
        Err(e) => return Err(format!("Could not enumerate libc symbols: {}", e)),
        Ok(_) => {  }
    }

    return match dlopen_addr {
        Some(addr) => Ok(addr),
        None => Err(format!("The symbol __libc_dlopen_mode was not found in the target libc"))
    };
}

pub fn inject(pid : i32, libpath : &String) -> Result<(), String> {
    let libc_entry = match find_libc(pid) {
        Some(entry) => entry,
        None => return Err(format!("Could not find libc in target process"))
    };
    println!("Target libc info:");
    println!("\tBase Address: {:#x}", libc_entry.base);
    println!("\tPath: {}", libc_entry.path);
    separator!();
    let dlopen_addr = match find_dlopen(&libc_entry) {
        Ok(addr) => addr,
        Err(e) => return Err(format!("Could not find dlopen in libc: {}", e))
    };
    separator!();

    return Ok(());
}