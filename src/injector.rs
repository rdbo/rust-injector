use regex::Regex;
use crate::proc::{enum_maps, MapsEntry};
use crate::elf::{enum_symbols};
use crate::elf::elfdefs::{ElfW_Ehdr, ElfW, Elf32_Ehdr, Elf64_Ehdr, ET_EXEC, ELFCLASS64, ELFCLASS32};
use std::fs::File;
use std::mem::{size_of, transmute};
use core::ffi::c_void;
use nix::libc::c_long;

use nix::unistd::Pid;
use nix::sys::ptrace;
use nix::sys::ptrace::AddressType;
use nix::sys::wait;
use nix::sys::signal::Signal;
use nix::errno::Errno;

pub const RTLD_LAZY : i32 = 0x1;
pub const RTLD_NOW : i32 = 0x2;
pub const RTLD_BINDING_MASK : i32 = 0x3;
pub const RTLD_NOLOAD : i32 = 0x4;
pub const RTLD_DEEPBIND : i32 = 0x8;
pub const RTLD_GLOBAL : i32 = 0x100;
pub const RTLD_LOCAL : i32 = 0;
pub const RTLD_NODELETE : i32 = 0x1000;

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

trait ByteFmt {
    fn bytearr(&self) -> String;
    fn bytestr(&self) -> String;
}

impl ByteFmt for Vec<u8> {
    fn bytearr(&self) -> String {
        let mut output = String::new();
        for i in 0..self.len() {
            if i != 0 {
                output = format!("{}, {:#x}", output, self[i]);
            } else {
                output = format!("{:#x}", self[i]);
            }
        }
        output = format!("[ {} ]", output);
        return output;
    }

    fn bytestr(&self) -> String {
        let mut output = String::new();
        for i in 0..self.len() {
            output = format!("{}\\x{:02x}", output, self[i]);
        }
        output = format!("\"{}\"", output);
        return output;
    }
}

fn ptwrite(pid : Pid, addr : u64, buf : &mut Vec<u8>) -> Result<(), Errno> {
    let buflen = buf.len();
    const datsiz : usize = size_of::<*mut c_void>();

    // Add padding for ptrace call (word-aligned)
    if buflen < datsiz {
        for i in 0..(datsiz - buflen) {
            buf.push(0);
        }
    } else if buflen % datsiz != 0 {
        for i in 0..datsiz - (buflen % datsiz) {
            buf.push(0);
        }
    }

    for i in 0..(buf.len() / datsiz) {
        let mut databuf : [u8;datsiz] = [0;datsiz];
        for j in 0..datsiz {
            // copy data section into databuf
            databuf[j] = buf[i * datsiz + j];
        }

        let data = unsafe {
            transmute::<[u8;datsiz], *mut c_void>(databuf)
        };
        match unsafe { ptrace::write(pid, (addr + (i * datsiz) as u64) as AddressType, data) } {
            Err(e) => return Err(e),
            _ => {  }
        }
    }

    return Ok(());
}

fn ptread(pid : Pid, addr : u64, size : usize) -> Result<Vec<u8>, Errno> {
    let mut buf : Vec<u8> = vec![];
    let mut size = size;
    const datsiz : usize = size_of::<c_long>();

    if size < datsiz {
        size = datsiz;
    } else if size % datsiz != 0 {
        let diff = datsiz - (size % datsiz);
        size += diff;
    }

    for i in 0..(size / datsiz) {
        let data = match ptrace::read(pid, (addr + (i * datsiz) as u64) as AddressType) {
            Ok(dat) => dat,
            Err(e) => return Err(e),
        };

        let databuf : [u8;datsiz] = unsafe {
            transmute::<c_long, [u8;datsiz]>(data)
        };
        buf.extend(databuf);
    }

    return Ok(buf);
}

fn call_dlopen(pid : i32, elf_class : u8, dlopen_addr : u64, libpath : &String, mode : i32) -> Result<u64, String> {
    let libpath = format!("{}\x00", libpath); // ensure null terminator
    let pid = Pid::from_raw(pid);
    println!("Attaching to process...");
    match ptrace::attach(pid) {
        Err(e) => return Err(format!("Could not attach to process {}: {}", pid, e)),
        _ => {  }
    }
    wait::wait();
    println!("Attached to process {}", pid);

    let old_regs = match ptrace::getregs(pid) {
        Ok(r) => r,
        Err(e) => {
            ptrace::detach(pid, None);
            return Err(format!("Could not get registers from process: {}", e));
        }
    };
    println!("Registers: {:x?}", old_regs);
    let mut regs = old_regs;

    let mut stackbuf : Vec<u8> = vec![];
    /*
    let modebuf = unsafe {
        transmute::<i32, [u8;size_of::<i32>()]>(mode)
    };
    stackbuf.extend(modebuf);
    */
    let pathbuf = libpath.as_bytes();
    stackbuf.extend(pathbuf);
    println!("Stack Buffer (size: {}): {}", stackbuf.len(), stackbuf.bytestr());

    let mut payload : Vec<u8> = vec![];
    if elf_class == ELFCLASS32 {
        /*
         * Payload
         * push ecx
         * push ebx
         * call eax
         * int3
         */
        payload.extend([0x51, 0x53, 0xff, 0xd0, 0xcc]);
    } else if elf_class == ELFCLASS64 {
        /*
         * Payload
         * call rax
         * int3
         */
        payload.extend([0xff, 0xd0, 0xcc]);
    } else {
        ptrace::detach(pid, None);
        return Err(format!("Invalid process ELF class"));
    }

    println!("Payload Buffer (size: {}): {}", payload.len(), payload.bytestr());

    regs.rsp -= stackbuf.len() as u64;
    regs.rsp &= 0xfffffffffffffff0; // align stack
    match ptwrite(pid, regs.rsp, &mut stackbuf) {
        Err(e) => {
            ptrace::detach(pid, None);
            return Err(format!("Unable to write stack buffer into target process: {}", e));
        }

        _ => {  }
    }
    println!("Stack buffer written into target process");

    if elf_class == ELFCLASS32 {
        regs.rax = dlopen_addr;
        regs.rbx = regs.rsp;
        regs.rcx = mode as u64;
    } else {
        regs.rax = dlopen_addr;
        regs.rdi = regs.rsp; // arg0
        regs.rsi = mode as u64; // arg1
    }

    let mut old_code = match ptread(pid, regs.rip, payload.len()) {
        Ok(buf) => buf,
        Err(e) => {
            ptrace::detach(pid, None);
            return Err(format!("Unable to read code buffer from target process: {}", e));
        }
    };
    println!("Code buffer read from target process");
    println!("Code: {}", old_code.bytestr());

    match ptwrite(pid, regs.rip, &mut payload) {
        Err(e) => {
            ptrace::detach(pid, None);
            return Err(format!("Unable to write payload buffer into target process: {}", e));
        }

        _ => {  }
    }
    println!("Payload injected into target process");

    match ptrace::setregs(pid, regs) {
        Err(e) => {
            ptwrite(pid, regs.rip, &mut old_code);
            ptrace::detach(pid, None);
            return Err(format!("Unable to set registers on target process: {}", e));
        }

        _ => {  }
    }
    println!("Set registers on target process");

    match ptrace::cont(pid, None) {
        Err(e) => {
            ptwrite(pid, regs.rip, &mut old_code);
            ptrace::setregs(pid, old_regs);
            ptrace::detach(pid, None);
            return Err(format!("Unable to continue target process: {}", e));
        }

        _ => {  }
    }
    println!("Continued target process");
    println!("Waiting for target process to stop...");
    wait::waitpid(pid, Some(wait::WaitPidFlag::WSTOPPED));
    println!("Target process stopped");

    let handle : Result<u64, String> = match ptrace::getregs(pid) {
        Ok(r) => {
            println!("Post injection registers: {:x?}", r);
            Ok(r.rax)
        },
        Err(e) => Err(format!("Unable to read registers from target process: {}", e))
    };

    ptwrite(pid, old_regs.rip, &mut old_code);
    ptrace::setregs(pid, old_regs);
    ptrace::detach(pid, None);

    return handle;
}

pub fn inject(pid : i32, elf_class : u8, libpath : &String) -> Result<(), String> {
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

    let libc_file = match File::open(libc_entry.path) {
        Ok(file) => file,
        Err(e) => return Err(format!("Could not open libc file"))
    };

    let handle = match call_dlopen(pid, elf_class, dlopen_addr, libpath, RTLD_LAZY) {
        Ok(h) => h,
        Err(e) => return Err(format!("Could not run dlopen on target process: {}", e))
    };

    println!("Library handle: {:#x}", handle);
    separator!();

    return Ok(());
}