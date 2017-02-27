extern crate libcapstone_sys;

use libcapstone_sys::*;

fn main() {
    let mut engine: csh;
    cs_open(CS_ARCH_X86, CS_MODE_64, &mut engine);
}