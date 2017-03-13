#![feature(untagged_unions)]

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::CStr;
use std::marker::PhantomData;
use std::os::raw::c_char;
use std::ptr;

/// Represents a Result from the capstone engine.
pub type CsResult<T> = Result<T, cs_err>;

type CsOptionValue = Option<usize>;

/// A utlity for configuring the Capstone engine.
pub struct Builder {
    arch: cs_arch,
    mode: cs_mode,
    syntax: CsOptionValue,
    detail: CsOptionValue,
    skipdata: CsOptionValue,
    skipdata_config: Option<cs_opt_skipdata>,
}

impl Builder {
    /// Returns a new `Builder` for the given `arch` and `mode`.
    pub fn new(arch: cs_arch, mode: cs_mode) -> Builder {
        Builder {
            arch: arch,
            mode: mode,
            syntax: None,
            detail: None,
            skipdata: None,
            skipdata_config: None,
        }
    }

    /// Create the Capstone engine with the configured options.
    pub fn build(self) -> CsResult<Capstone> {
        let engine = Capstone::new(self.arch, self.mode)?;

        if let Some(opt) = self.syntax {
            engine.option(CS_OPT_SYNTAX, opt)?;
        }

        if let Some(opt) = self.detail {
            engine.option(CS_OPT_DETAIL, opt)?;
        }

        if let Some(opt) = self.skipdata {
            engine.option(CS_OPT_SKIPDATA, opt)?;
        }

        if let Some(opt) = self.skipdata_config {
            if self.skipdata.is_none() || self.skipdata.unwrap() != CS_OPT_ON as _ {
                engine.option(CS_OPT_SKIPDATA, CS_OPT_ON as _)?;
            }

            engine.option(CS_OPT_SKIPDATA_SETUP, &opt as *const _ as _)?;
        }

        Ok(engine)
    }

    /// Set the syntax for the engine.
    pub fn syntax(mut self, syntax: cs_opt_value) -> Builder {
        self.syntax = Some(syntax as _);
        self
    }

    /// Set whether detail mode should be on.
    pub fn detail(mut self, detail: cs_opt_value) -> Builder {
        self.detail = Some(detail as _);
        self
    }

    /// Whether data encountered in code should be skipped.
    pub fn skipdata(mut self, doit: cs_opt_value) -> Builder {
        self.skipdata = Some(doit as _);
        self
    }

    /// Setup configuration for skipdata.
    /// Userdata will always be a nullptr for now.
    pub fn skipdata_config(mut self,
                           mnemonic: Option<&'static str>,
                           callback: cs_skipdata_cb_t)
                           -> Builder {
        self.skipdata_config = Some(cs_opt_skipdata {
                                        mnemonic: mnemonic.unwrap_or(".byte").as_ptr() as _,
                                        callback: callback,
                                        user_data: ptr::null_mut(),
                                    });
        self
    }
}

/// An instance of `Capstone` represents an instance of the capstone engine.
pub struct Capstone {
    handle: csh,
}

impl Capstone {
    /// `new` returns a new instance of the capstone engine with the given
    /// `arch`itecture and `mode` used.
    pub fn new(arch: cs_arch, mode: cs_mode) -> CsResult<Capstone> {
        let mut handle = 0;
        let err = unsafe { cs_open(arch, mode, &mut handle) };
        if err != CS_ERR_OK {
            Err(err)
        } else {
            Ok(Capstone { handle: handle })
        }
    }

    /// `disasm` disassembles the given `code` at the given `address`.
    /// It disassembles `count` instructions. If `count` is 0, then as many
    /// instructions as possible are disassembled.
    pub fn disasm(&self, code: &[u8], address: u64, count: usize) -> CsResult<Instructions> {
        let mut instructions: *mut cs_insn = ptr::null_mut();
        let count = unsafe {
            cs_disasm(self.handle,
                      code.as_ptr(),
                      code.len(),
                      address,
                      count,
                      &mut instructions)
        };

        if count == 0 {
            let err = unsafe { cs_errno(self.handle) };
            debug_assert_ne!(err, CS_ERR_OK);
            Err(err)
        } else {
            Ok(Instructions::from_raw(instructions, count))
        }
    }

    /// `disasm_all` disassembles as many instructions as possible in the given `code`
    /// at the given `address`.
    pub fn disasm_all(&self, code: &[u8], address: u64) -> CsResult<Instructions> {
        self.disasm(code, address, 0)
    }

    /// `error` returns the current error as a String.
    /// This may return `None`, if there is no error, or the conversion to a String failed.
    pub fn error(&self) -> Option<String> {
        let err = unsafe { cs_errno(self.handle) };
        if err != CS_ERR_OK {
            to_string(unsafe { cs_strerror(err) })
        } else {
            None
        }
    }

    /// Returns the name of the given group.
    pub fn group_name(&self, group_id: u32) -> Option<String> {
        to_string(unsafe { cs_group_name(self.handle, group_id) })
    }

    /// Returns whether the given instruction belongs to the given group.
    pub fn insn_group(&self, insn: &cs_insn, group_id: u32) -> bool {
        unsafe { cs_insn_group(self.handle, insn, group_id) }
    }

    /// Returns the name of the given instruction.
    pub fn insn_name(&self, insn_id: u32) -> Option<String> {
        to_string(unsafe { cs_insn_name(self.handle, insn_id) })
    }

    /// `option` sets the given option `type_` to the given `value`.
    pub fn option(&self, type_: cs_opt_type, value: usize) -> CsResult<()> {
        let err = unsafe { cs_option(self.handle, type_, value) };
        if err != CS_ERR_OK { Err(err) } else { Ok(()) }
    }

    /// Returns the name of the given register.
    pub fn reg_name(&self, reg_id: u32) -> Option<String> {
        to_string(unsafe { cs_reg_name(self.handle, reg_id) })
    }

    /// Returns whether the given register is read by the given instruction.
    pub fn reg_read(&self, insn: &cs_insn, reg_id: u32) -> bool {
        unsafe { cs_reg_read(self.handle, insn, reg_id) }
    }

    /// Returns whether the given register is written by the given instruction.
    pub fn reg_write(&self, insn: &cs_insn, reg_id: u32) -> bool {
        unsafe { cs_reg_write(self.handle, insn, reg_id) }
    }
}

impl Drop for Capstone {
    fn drop(&mut self) {
        let err = unsafe { cs_close(&mut self.handle) };
        if err != CS_ERR_OK {
            panic!("Error while calling cs_close: {:?}", err)
        }
    }
}

/// `Instructions` represents a number of disassembled instructions.
pub struct Instructions {
    instructions: *mut cs_insn,
    count: usize,
}

impl Instructions {
    fn from_raw(p: *mut cs_insn, count: usize) -> Instructions {
        Instructions {
            instructions: p,
            count: count,
        }
    }

    /// Returns true if there are no instructions.
    /// This actually will never be true but clippy wants there to be a `is_empty` method
    /// when there is a `len` method.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Returns an `PointerIter` to iterate over all the instructions.
    pub fn iter(&self) -> PointerIter<cs_insn> {
        PointerIter::new(self.instructions, self.count)
    }

    /// Returns the number of instructions owned by this struct.
    pub fn len(&self) -> usize {
        self.count
    }
}

impl Drop for Instructions {
    fn drop(&mut self) {
        unsafe { cs_free(self.instructions, self.count) }
    }
}

/// Returns whether the requested feature is supported.
/// `query` can be one of the `CS_ARCH_`* values or `CS_ARCH_ALL` + 1 to check the diet mode.
pub fn support(query: cs_arch) -> bool {
    unsafe { cs_support(query as _) }
}

/// Returns the major version, minor version and the combined version.
pub fn version() -> (i32, i32, u32) {
    let mut major = 0;
    let mut minor = 0;
    (major, minor, unsafe { cs_version(&mut major, &mut minor) })
}

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

    /// Returns an iterator over the registers read by this instruction.
    pub fn regs_read_iter(&self) -> PointerIter<u8> {
        PointerIter::new(self.regs_read.as_ptr(), self.regs_read_count as _)
    }

    /// Returns an iterator over the registers written to by this instruction.
    pub fn regs_write_iter(&self) -> PointerIter<u8> {
        PointerIter::new(self.regs_write.as_ptr(), self.regs_write_count as _)
    }

    /// Returns an iterator over the groups this instruction belongs to.
    pub fn groups_iter(&self) -> PointerIter<u8> {
        PointerIter::new(self.groups.as_ptr(), self.groups_count as _)
    }
}

impl cs_insn {
    pub fn get_mnemonic(&self) -> Option<String> {
        to_string(self.mnemonic.as_ptr())
    }

    pub fn get_op_str(&self) -> Option<String> {
        to_string(self.op_str.as_ptr())
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

fn to_string(p: *const c_char) -> Option<String> {
    if p.is_null() {
        None
    } else {
        Some(unsafe { CStr::from_ptr(p) }.to_string_lossy().into_owned())
    }
}
