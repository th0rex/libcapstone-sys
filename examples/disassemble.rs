extern crate libcapstone_sys;

use libcapstone_sys::*;

fn main() {
    assert!(support(CS_ARCH_X86));

    let engine = Builder::new(CS_ARCH_X86, CS_MODE_64)
        .syntax(CS_OPT_SYNTAX_INTEL)
        .detail(CS_OPT_ON)
        .build()
        .expect("Could not create capstone engine");

    let instructions = engine.disasm_all(&[0x31, 0xed, 0x49, 0x89, 0xd1], 0x4a7aa0)
        .expect("Could not disassemble instructions");

    for instruction in instructions.iter() {
        println!("Instruction: id={}, address={}, size={}, bytes={:?}, mnemonic={}, op_str={}, \
                  detail={:?}",
                 instruction.id,
                 instruction.address,
                 instruction.size,
                 instruction.bytes,
                 instruction.get_mnemonic().unwrap(),
                 instruction.get_op_str().unwrap(),
                 instruction.detail);
    }
}
