#![feature(untagged_unions)]

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::fmt;
impl fmt::Debug for cs_insn {
    fn fmt(&self, f: &mut fmt::Formatter) -> 
     fmt::Result {
        f.debug_struct("cs_insn")
            .field("id", &self.id)
            
     .field("address", &self.address)
            .field("size", &self.size)
            
     .field("bytes", &self.bytes)
            .field("mnemonic", &self.mnemonic)
            
     .field("detail", &self.detail)
            .finish()
    }
}