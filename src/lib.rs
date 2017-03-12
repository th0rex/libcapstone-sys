#![feature(untagged_unions)]

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::CStr;
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

impl cs_arm64 {
    pub fn operand_iter(&self) -> PointerIter<cs_arm64_op> {
        PointerIter::new(self.operands.as_ptr(), self.op_count as _)
    }
}

impl cs_mips {
    pub fn operand_iter(&self) -> PointerIter<cs_mips_op> {
        PointerIter::new(self.operands.as_ptr(), self.op_count as _)
    }
}

impl cs_ppc {
    pub fn operand_iter(&self) -> PointerIter<cs_ppc_op> {
        PointerIter::new(self.operands.as_ptr(), self.op_count as _)
    }
}

impl cs_sparc {
    pub fn operand_iter(&self) -> PointerIter<cs_sparc_op> {
        PointerIter::new(self.operands.as_ptr(), self.op_count as _)
    }
}

impl cs_sysz {
    pub fn operand_iter(&self) -> PointerIter<cs_sysz_op> {
        PointerIter::new(self.operands.as_ptr(), self.op_count as _)
    }
}

impl cs_x86 {
    pub fn operand_iter(&self) -> PointerIter<cs_x86_op> {
        PointerIter::new(self.operands.as_ptr(), self.op_count as _)
    }
}

impl cs_xcore {
    pub fn operand_iter(&self) -> PointerIter<cs_xcore_op> {
        PointerIter::new(self.operands.as_ptr(), self.op_count as _)
    }
}

impl cs_detail {
    /// Returns a reference to the `x86` field of the union.
    /// It is the responsibility of the caller to ensure that the field may be used.
    pub fn get_x86(&self) -> &cs_x86 {
        unsafe { &self.__bindgen_anon_1.x86 }
    }

    /// Returns a reference to the `arm64` field of the union.
    /// It is the responsibility of the caller to ensure that the field may be used.
    pub fn get_arm64(&self) -> &cs_arm64 {
        unsafe { &self.__bindgen_anon_1.arm64 }
    }

    /// Returns a reference to the `arm` field of the union.
    /// It is the responsibility of the caller to ensure that the field may be used.
    pub fn get_arm(&self) -> &cs_arm {
        unsafe { &self.__bindgen_anon_1.arm }
    }

    /// Returns a reference to the `mips` field of the union.
    /// It is the responsibility of the caller to ensure that the field may be used.
    pub fn get_mips(&self) -> &cs_mips {
        unsafe { &self.__bindgen_anon_1.mips }
    }

    /// Returns a reference to the `ppc` field of the union.
    /// It is the responsibility of the caller to ensure that the field may be used.
    pub fn get_ppc(&self) -> &cs_ppc {
        unsafe { &self.__bindgen_anon_1.ppc }
    }

    /// Returns a reference to the `sparc` field of the union.
    /// It is the responsibility of the caller to ensure that the field may be used.
    pub fn get_sparc(&self) -> &cs_sparc {
        unsafe { &self.__bindgen_anon_1.sparc }
    }

    /// Returns a reference to the `sysz` field of the union.
    /// It is the responsibility of the caller to ensure that the field may be used.
    pub fn get_sysz(&self) -> &cs_sysz {
        unsafe { &self.__bindgen_anon_1.sysz }
    }

    /// Returns a reference to the `xcore` field of the union.
    /// It is the responsibility of the caller to ensure that the field may be used.
    pub fn get_xcore(&self) -> &cs_xcore {
        unsafe { &self.__bindgen_anon_1.xcore }
    }

    pub fn regs_read_iter(&self) -> PointerIter<u8> {
        PointerIter::new(self.regs_read.as_ptr(), self.regs_read_count as _)
    }

    pub fn regs_write_iter(&self) -> PointerIter<u8> {
        PointerIter::new(self.regs_write.as_ptr(), self.regs_write_count as _)
    }

    pub fn groups_iter(&self) -> PointerIter<u8> {
        PointerIter::new(self.groups.as_ptr(), self.groups_count as _)
    }
}

impl cs_insn {
    pub fn get_mnemonic(&self) -> String {
        unsafe { CStr::from_ptr(self.mnemonic.as_ptr()) }.to_string_lossy().into_owned()
    }

    pub fn get_op_str(&self) -> String {
        unsafe { CStr::from_ptr(self.op_str.as_ptr()) }.to_string_lossy().into_owned()
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
