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

pub trait ElfW_Ehdr {
    fn new() -> Self;
}

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

impl ElfW_Ehdr for Elf32_Ehdr {
    fn new() -> Self {
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

impl Elf64_Ehdr {
    pub fn new() -> Self {
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

pub enum ElfW<A, B> {
    Elf32(A),
    Elf64(B)
}
