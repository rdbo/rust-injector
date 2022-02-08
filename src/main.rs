use std::env;
use std::path::Path;
use std::fs::File;

mod proc;
mod elf;
mod injector;
use elf::elfdefs::*;
use nix::unistd::geteuid;

macro_rules! separator {
    () => {
        println!("====================");
    }
}

fn help() {
    println!("usage: ./injector [-n NAME][-p PID] SHARED_LIB");
}

fn main() {
    println!("[ Rust Injector ] by rdbo");
    separator!();

    assert!(geteuid().is_root(), "Please run as root");

    let mut args : Vec<String> = env::args().collect();
    args = args[1..].to_vec(); // remove first argument (program)
    if args.len() != 3 {
        help();
        return;
    }

    let mut prev = String::new();
    let mut libpath = String::new();
    let mut pid : i32 = 0;
    let mut name = String::new();
    let mut fname = String::new();

    for arg in args {
        match arg.as_str() {
            "-n" | "-p" | "-f" => { prev = arg; continue; },
            _ => {
                match prev.as_str() {
                    "-n" => { name = arg; prev.clear() },
                    "-p" => { pid = arg.parse().unwrap_or(0); prev.clear() },
                    "-f" => { fname = arg; prev.clear() },
                    _ => libpath = arg
                }
            }
        }
    }

    if libpath == "" || (pid <= 0 && name == "" && fname == "") {
        help();
        return;
    }

    let lib_exists = Path::new(&libpath).is_file();
    assert!(lib_exists, "The library \"{}\" does not exist", libpath);

    if pid <= 0 {
        if name != "" {
            pid = match proc::pid_from_name(&name) {
                Some(p) => p,
                _ => panic!("Unable to get PID from name '{}'", name)
            }
        } else {
            pid = match proc::pid_from_fname(&fname) {
                Some(p) => p,
                _ => panic!("Unable to get PID from filename '{}'", fname)
            }
        }
    }
    
    if name == "" {
        if let Some(option) = proc::name_from_pid(pid) {
            name = option;
        }
    }
    
    /* The 'filename' entered by the user does not need to be a full path,
     * therefore it will be always retrieved (to have the full path)
     */
    if let Some(option) = proc::exepath_from_pid(pid) {
        fname = option;
    } else {
        panic!("Unable to retrieve process executable path!");
    }

    println!("General info: ");
    println!("\tLibrary path: {}", libpath);
    println!("\tProcess ID: {}", pid);
    println!("\tProcess Name: {}", name);
    println!("\tProcess Filename: {}", fname);
    separator!();

    let libfile = File::open(&libpath).expect("Unable to open library file");
    let lib_ehdr = match elf::read_ehdr(&libfile) {
        Ok(ehdr) => ehdr,
        Err(e) => panic!("Unable to read library ELF file: {}", e)
    };

    println!("Library ELF info: ");
    println!("\tClass: {}", lib_ehdr.get_class());
    println!("\tMagic: {}", String::from_utf8_lossy(lib_ehdr.get_magic().as_slice()));
    println!("\tHeader: {:?}", lib_ehdr);
    separator!();

    let exefile = File::open(fname).expect("Unable to open process executable file");
    let exe_ehdr = match elf::read_ehdr(&exefile) {
        Ok(ehdr) => ehdr,
        Err(e) => panic!("Unable to read process ELF file: {}", e)
    };

    println!("Process ELF info: ");
    println!("\tClass: {}", exe_ehdr.get_class());
    println!("\tMagic: {}", String::from_utf8_lossy(exe_ehdr.get_magic().as_slice()));
    println!("\tHeader: {:?}", exe_ehdr);
    separator!();

    assert!(
        lib_ehdr.get_class() == exe_ehdr.get_class(),
        "The ELF classes from the library and the process don't match. Make sure they are the same architecture!"
    );

    match injector::inject(pid, &libpath) {
        Some(_) => println!("Injected successfully!"),
        None => println!("Unable to inject")
    }
}
