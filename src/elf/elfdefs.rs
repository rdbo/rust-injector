/*
 * Definitions from the elf.h C header
 */

#![allow(non_camel_case_types)]
#![allow(dead_code)]
pub type Elf32_Half = u16;
pub type Elf64_Half = u16;

pub type Elf32_Word = u32;
pub type Elf32_Sword = i32;
pub type Elf64_Word = u32;
pub type Elf64_Sword = i32;

pub type Elf32_Xword = u64;
pub type Elf32_Sxword = i64;
pub type Elf64_Xword = u64;
pub type Elf64_Sxword = i64;

pub type Elf32_Addr = u32;
pub type Elf64_Addr = u64;

pub type Elf32_Off = u32;
pub type Elf64_Off = u64;

pub type Elf32_Section = u16;
pub type Elf64_Section = u16;

pub const EI_NIDENT : usize = 16;

#[derive(Debug)]
#[repr(C)]
pub struct Elf32_Ehdr {
    pub e_ident : [u8; EI_NIDENT],
    pub e_type : Elf32_Half,
    pub e_machine : Elf32_Half,
    pub e_version : Elf32_Word,
    pub e_entry : Elf32_Addr,
    pub e_phoff : Elf32_Off,
    pub e_shoff : Elf32_Off,
    pub e_flags : Elf32_Word,
    pub e_ehsize : Elf32_Half,
    pub e_phentsize : Elf32_Half,
    pub e_phnum : Elf32_Half,
    pub e_shentsize : Elf32_Half,
    pub e_shnum : Elf32_Half,
    pub e_shstrndx : Elf32_Half
}

#[derive(Debug)]
#[repr(C)]
pub struct Elf64_Ehdr {
    pub e_ident : [u8; EI_NIDENT],
    pub e_type : Elf64_Half,
    pub e_machine : Elf64_Half,
    pub e_version : Elf64_Word,
    pub e_entry : Elf64_Addr,
    pub e_phoff : Elf64_Off,
    pub e_shoff : Elf64_Off,
    pub e_flags : Elf64_Word,
    pub e_ehsize : Elf64_Half,
    pub e_phentsize : Elf64_Half,
    pub e_phnum : Elf64_Half,
    pub e_shentsize : Elf64_Half,
    pub e_shnum : Elf64_Half,
    pub e_shstrndx : Elf64_Half
}

pub const EI_MAG0 : usize = 0;
pub const ELFMAG0 : i8 = 0x7f as i8;

pub const EI_MAG1 : usize = 1;
pub const ELFMAG1 : i8 = 'E' as i8;

pub const EI_MAG2 : usize = 2;
pub const ELFMAG2 : i8 = 'L' as i8;

pub const EI_MAG3 : usize = 3;
pub const ELFMAG3 : i8 = 'F' as i8;

pub const SELFMAG : usize = 4;
pub const ELFMAG : &[u8] = "\x7fELF".as_bytes();

pub const EI_CLASS : usize = 4;
pub const ELFCLASSNONE : u8 = 0;
pub const ELFCLASS32 : u8 = 1;
pub const ELFCLASS64 : u8 = 2;
pub const ELFCLASSNUM : u8 = 3;

#[derive(Debug)]
#[repr(C)]
struct Elf32_Shdr {
    sh_name : Elf32_Word,
    sh_type : Elf32_Word,
    sh_flags : Elf32_Word,
    sh_addr : Elf32_Addr,
    sh_offset : Elf32_Off,
    sh_size : Elf32_Word,
    sh_link : Elf32_Word,
    sh_info : Elf32_Word,
    sh_addralign : Elf32_Word,
    sh_entsize : Elf32_Word
}

#[derive(Debug)]
#[repr(C)]
struct Elf64_Shdr {
    sh_name : Elf64_Word,
    sh_type : Elf64_Word,
    sh_flags : Elf64_Xword,
    sh_addr : Elf64_Addr,
    sh_offset : Elf64_Off,
    sh_size : Elf64_Xword,
    sh_link : Elf64_Word,
    sh_info : Elf64_Word,
    sh_addralign : Elf64_Xword,
    sh_entsize : Elf64_Xword,
}

/********************/

impl Elf32_Shdr {
    pub fn new() -> Self {
        return Elf32_Shdr {
            sh_name: 0,
            sh_type: 0,
            sh_flags: 0,
            sh_addr: 0,
            sh_offset: 0,
            sh_size: 0,
            sh_link: 0,
            sh_info: 0,
            sh_addralign: 0,
            sh_entsize: 0
        }
    }
}

impl Elf64_Shdr {
    pub fn new() -> Self {
        return Elf64_Shdr {
            sh_name: 0,
            sh_type: 0,
            sh_flags: 0,
            sh_addr: 0,
            sh_offset: 0,
            sh_size: 0,
            sh_link: 0,
            sh_info: 0,
            sh_addralign: 0,
            sh_entsize: 0
        }
    }
}

impl Elf32_Ehdr {
    pub fn new() -> Self {
        return Elf32_Ehdr {
            e_ident: [0; EI_NIDENT],
            e_type: 0,
            e_machine: 0,
            e_version: 0,
            e_entry: 0,
            e_phoff: 0,
            e_shoff: 0,
            e_flags: 0,
            e_ehsize: 0,
            e_phentsize: 0,
            e_phnum: 0,
            e_shentsize: 0,
            e_shnum: 0,
            e_shstrndx: 0
        };
    }
}

impl Elf64_Ehdr {
    fn new() -> Self {
        return Elf64_Ehdr {
            e_ident: [0; EI_NIDENT],
            e_type: 0,
            e_machine: 0,
            e_version: 0,
            e_entry: 0,
            e_phoff: 0,
            e_shoff: 0,
            e_flags: 0,
            e_ehsize: 0,
            e_phentsize: 0,
            e_phnum: 0,
            e_shentsize: 0,
            e_shnum: 0,
            e_shstrndx: 0
        };
    }
}

use std::mem::{size_of, transmute};
use std::os::unix::fs::FileExt;
use std::fs::File;
use std::io::{Seek, SeekFrom, BufRead, BufReader};

pub trait ElfW_Ehdr {
    fn get_class(&self) -> u8;
    fn get_magic(&self) -> Vec<u8>;
    fn enum_sections<T>(&self, file : &File, callback : T) -> Option<()> where T : FnMut(String, u64, u64, u64) -> bool;
}

impl ElfW_Ehdr for Elf32_Ehdr {
    fn get_class(&self) -> u8 {
        return self.e_ident[EI_CLASS];
    }

    fn get_magic(&self) -> Vec<u8> {
        return Vec::from(&self.e_ident[0..SELFMAG]);
    }

    fn enum_sections<T>(&self, file : &File, mut callback : T) -> Option<()> where T : FnMut(String, u64, u64, u64) -> bool {
        // Get shstrtab
        let mut shstrtab_off = self.e_shoff + (self.e_shstrndx * self.e_shentsize) as Elf32_Off;
        let mut shbuf : [u8;size_of::<Elf32_Shdr>()] = [0;size_of::<Elf32_Shdr>()];
        file.read_exact_at(&mut shbuf, shstrtab_off as u64).ok()?;
        let shstrtab = unsafe {
            transmute::<[u8;size_of::<Elf32_Shdr>()], Elf32_Shdr>(shbuf)
        };

        shstrtab_off = shstrtab.sh_offset;
        println!("ELF shstrtab offset: {:#x}", shstrtab_off);

        // Loop through sections
        for i in 0..self.e_shnum {
            file.read_exact_at(&mut shbuf, (self.e_shoff + (i * self.e_shentsize) as Elf32_Off) as u64).ok()?;
            let shdr = unsafe {
                transmute::<[u8;size_of::<Elf32_Shdr>()], Elf32_Shdr>(shbuf)
            };

            let mut section_name_buf : Vec<u8> = vec![];
            let mut reader = BufReader::new(file);
            reader.seek(SeekFrom::Start((shstrtab_off + shdr.sh_name) as u64));
            reader.read_until(b'\x00', &mut section_name_buf);

            let section_name = String::from_utf8_lossy(&section_name_buf).to_string();
            if !callback(section_name, shdr.sh_offset as u64, shdr.sh_entsize as u64, shdr.sh_size as u64) {
                break;
            }
        }
        return Some(());
    }
}

impl ElfW_Ehdr for Elf64_Ehdr {
    fn get_class(&self) -> u8 {
        return self.e_ident[EI_CLASS];
    }

    fn get_magic(&self) -> Vec<u8> {
        return Vec::from(&self.e_ident[0..SELFMAG]);
    }

    fn enum_sections<T>(&self, file : &File, mut callback : T) -> Option<()> where T : FnMut(String, u64, u64, u64) -> bool {
        // Get shstrtab
        let mut shstrtab_off = self.e_shoff + (self.e_shstrndx * self.e_shentsize) as Elf64_Off;
        let mut shbuf : [u8;size_of::<Elf64_Shdr>()] = [0;size_of::<Elf64_Shdr>()];
        file.read_exact_at(&mut shbuf, shstrtab_off as u64).ok()?;
        let shstrtab = unsafe {
            transmute::<[u8;size_of::<Elf64_Shdr>()], Elf64_Shdr>(shbuf)
        };

        shstrtab_off = shstrtab.sh_offset;
        println!("ELF shstrtab offset: {:#x}", shstrtab_off);

        // Loop through sections
        for i in 0..self.e_shnum {
            file.read_exact_at(&mut shbuf, (self.e_shoff + (i * self.e_shentsize) as Elf64_Off) as u64).ok()?;
            let shdr = unsafe {
                transmute::<[u8;size_of::<Elf64_Shdr>()], Elf64_Shdr>(shbuf)
            };

            let mut section_name_buf : Vec<u8> = vec![];
            let mut reader = BufReader::new(file);
            reader.seek(SeekFrom::Start((shstrtab_off + shdr.sh_name as Elf64_Off) as u64));
            reader.read_until(b'\x00', &mut section_name_buf);

            let section_name = String::from_utf8_lossy(&section_name_buf).to_string();
            if !callback(section_name, shdr.sh_offset as u64, shdr.sh_entsize as u64, shdr.sh_size as u64) {
                break;
            }
        }
        return Some(());
    }
}

#[derive(Debug)]
pub enum ElfW<A, B> {
    Elf32(A),
    Elf64(B)
}

extern crate lib;
use lib::elfw;

impl<A, B> ElfW_Ehdr for ElfW<A, B>
where A : ElfW_Ehdr, B : ElfW_Ehdr {
    fn get_class(&self) -> u8 {
        return elfw!(e.get_class());
    }

    fn get_magic(&self) -> Vec<u8> {
        return elfw!(e.get_magic());
    }

    fn enum_sections<T>(&self, file : &File, mut callback : T) -> Option<()> where T : FnMut(String, u64, u64, u64) -> bool {
        return elfw!(e.enum_sections(file, callback));
    }
}
