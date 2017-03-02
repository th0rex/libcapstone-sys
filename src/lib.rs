#![feature(untagged_unions)]

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::marker::PhantomData;

/// `PointerIter` iterates over an array of things that are
/// pointed to by an pointer.
pub struct PointerIter<'a, T: 'a> {
    ptr: *const T,
    count: usize,
    current: usize,
    lifetime: PhantomData<&'a T>,
}

impl<'a, T: 'a> PointerIter<'a, T> {
    pub fn new(ptr: *const T, count: usize) -> Self {
        PointerIter {
            ptr: ptr,
            count: count,
            current: 0,
            lifetime: PhantomData,
        }
    }
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

impl cs_arm {
    pub fn operand_iter(&self) -> PointerIter<cs_arm_op> {
        PointerIter::new(self.operands.as_ptr(), self.op_count as _)
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
