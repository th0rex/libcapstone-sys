#![feature(untagged_unions)]

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// `PointerIter` iterates over an array of things that are
/// pointed to by an pointer.
pub struct PointerIter<'a, T: 'a> {
    ptr: &'a *const T,
    count: usize,
    current: usize,
}

impl<'a, T: 'a> Iterator for PointerIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.current >= self.count {
            None
        } else {
            let object = unsafe { &*self.ptr.offset(self.current as _) };
            self.current += 1;
            Some(object)
        }
    }
}

use std::fmt;
impl fmt::Debug for cs_insn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
