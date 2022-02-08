use regex::Regex;
use crate::proc::{enum_maps, MapsEntry};

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

pub fn inject(pid : i32, libpath : &String) -> Option<()> {
    let libc_entry = match find_libc(pid) {
        Some(entry) => entry,
        None => { println!("Unable to find libc in target process"); return None; }
    };
    println!("Target libc info:");
    println!("\tBase Address: {:#x}", libc_entry.base);
    println!("\tEnd Address: {:#x}", libc_entry.end);
    println!("\tFlags: {:?}", libc_entry.flags);
    println!("\tPath: {}", libc_entry.path);
    separator!();

    return Some(());
}