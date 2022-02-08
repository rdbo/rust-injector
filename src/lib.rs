extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn elfw(code : TokenStream) -> TokenStream {
    return format!("
        match self {{
            ElfW::Elf32(e) => {0},
            ElfW::Elf64(e) => {0}
        }}
    ", code).parse().unwrap();
}
