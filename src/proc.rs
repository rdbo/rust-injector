use std::path::Path;
use std::fs::{read_dir, File, read_link};
use std::io::{BufRead, BufReader};
use regex::Regex;
use nix::sys::mman::ProtFlags;

#[derive(Debug)]
pub struct MapsEntry {
    pub base : usize,
    pub end : usize,
    pub flags : ProtFlags,
    pub path : String
}

pub fn enum_pids<F>(mut callback : F) -> Option<()> where F : FnMut(i32) -> bool {
    let procfs = Path::new("/proc");
    let dir = read_dir(procfs).ok()?;

    for entry in dir {
        let entry = match entry {
            Ok(e) => e,
            _ => continue
        };

        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let path = match entry.file_name().into_string() {
            Ok(ref s) => String::from(s),
            _ => continue
        };

        let curpid : i32 = match path.parse() {
            Ok(p) => p,
            _ => continue
        };

        if !callback(curpid) {
            break;
        }
    }

    return Some(());
}

pub fn enum_maps<F>(pid : i32, mut callback : F) -> Option<()>
where F : FnMut(MapsEntry) -> bool {
    let maps_file = File::open(format!("/proc/{}/maps", pid)).ok()?;
    let reader = BufReader::new(maps_file);
    let re = Regex::new(r"([0-9a-f]+)-([0-9a-f]+)\s+([rwxp\-]+).*\s+(/.*)").ok()?;

    for line in reader.lines() {
        let line = line.ok()?;
        let caps = match re.captures(line.as_str()) {
            Some(c) => c,
            _ => continue
        };
        
        let base_addr = usize::from_str_radix(&caps[1], 16).unwrap();
        let end_addr = usize::from_str_radix(&caps[2], 16).unwrap();
        let mut flags = ProtFlags::empty();
        for c in String::from(&caps[3]).chars() {
            flags = match c {
                'r' => flags | ProtFlags::PROT_READ,
                'w' => flags | ProtFlags::PROT_WRITE,
                'x' => flags | ProtFlags::PROT_EXEC,
                _ => continue
            }
        }
        let path = String::from(&caps[4]);
        let entry = MapsEntry{ base: base_addr, end: end_addr, flags: flags, path: path };

        if !callback(entry) {
            break;
        }
    }

    return Some(());
}

pub fn name_from_pid(pid : i32) -> Option<String> {
    let mut name = None;
    let status_path = format!("/proc/{}/status", pid);
    let status_file = File::open(status_path).ok()?;
    let reader = BufReader::new(status_file);

    for line in reader.lines() {
        let line = line.ok()?;
        if !line.starts_with("Name:") {
            continue;
        }

        if let Some(index) = line.find('\t') {
            name = Some(line[index + 1 ..].to_string());
        }

        break;
    }

    return name;
}

pub fn pid_from_name(name : &String) -> Option<i32> {
    let mut pid = None;
    
    enum_pids(|curpid : i32| {
        if let Some(curname) = name_from_pid(curpid) {
            if curname == *name {
                pid = Some(curpid);
                return false;
            }
        }

        return true;
    });

    return pid;
}

pub fn exepath_from_pid(pid : i32) -> Option<String> {
    let sym_path = format!("/proc/{}/exe", pid);
    let real_path = read_link(sym_path).ok()?;
    return Some(String::from(real_path.to_str()?));
}

pub fn pid_from_fname(fname : &String) -> Option<i32> {
    let mut pid = None;
    let fname = format!("/{}", fname);
    
    enum_pids(|curpid : i32| {
        if let Some(curpath) = exepath_from_pid(curpid) {
            if curpath.ends_with(&fname) {
                pid = Some(curpid);
                return false;
            }
        }

        return true;
    });

    return pid;
}
