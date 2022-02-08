use regex::Regex;
use crate::proc::{enum_maps, MapsEntry};
use crate::elf::enum_symbols;
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

fn find_dlopen(libc_entry : &MapsEntry) -> Result<usize, String> {
    let libc_file = match File::open(&libc_entry.path) {
        Ok(f) => f,
        Err(e) => return Err(format!("Could not open libc file: {}", e))
    };
    
    let result = enum_symbols(&libc_file, |symbol : String, value : usize| {
        println!("{} : {:#x}", symbol, value);
    });

    match result {
        Err(e) => return Err(format!("Could not enumerate libc symbols: {}", e)),
        Ok(_) => {  }
    }

    return Ok(0);
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

    return Ok(());
}