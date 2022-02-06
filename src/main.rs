use std::env;
use std::path::Path;

mod proc;

fn help() {
    println!("usage: ./injector [-n NAME][-p PID] SHARED_LIB");
}

fn main() {
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
    if !lib_exists {
        println!("The library '{}' does not exist", libpath);
        return;
    }

    if pid <= 0 {
        if name != "" {
            pid = match proc::pid_from_name(&name) {
                Some(p) => p,
                _ => { println!("Unable to get PID from name '{}'", name); return; }
            }
        } else {
            pid = match proc::pid_from_fname(&fname) {
                Some(p) => p,
                _ => { println!("Unable to get PID from filename '{}'", fname); return; }
            }
        }
    }
    
    if name == "" {
        if let Some(option) = proc::name_from_pid(pid) {
            name = option;
        }
    }
    
    if fname == "" {
        if let Some(option) = proc::exepath_from_pid(pid) {
            fname = option;
        }
    }

    println!("Libpath: {}", libpath);
    println!("PID: {}", pid);
    println!("Name: {}", name);
    println!("Filename: {}", fname);
}
