use std::ffi::{c_char, c_void};

use bochscpu::opcode::*;
use bochscpu::Address;

#[allow(non_camel_case_types)]
pub type bochscpu_instr_t = *const c_void;

#[no_mangle]
pub unsafe extern "C" fn bochscpu_instr_bx_opcode(p: bochscpu_instr_t) -> u32 {
    instr_bx_opcode(p)
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_instr_imm16(p: bochscpu_instr_t) -> u16 {
    instr_imm16(p)
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_instr_imm32(p: bochscpu_instr_t) -> u32 {
    instr_imm32(p)
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_instr_imm64(p: bochscpu_instr_t) -> u64 {
    instr_imm64(p)
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_instr_src(p: bochscpu_instr_t) -> u32 {
    instr_src(p)
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_instr_dst(p: bochscpu_instr_t) -> u32 {
    instr_dst(p)
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_opcode_disasm(
    is32: u32,
    is64: u32,
    cs_base: *mut Address,
    ip: *mut Address,
    instr_bytes: *mut u8,
    distbuf: *const c_char,
    disasm_style: DisasmStyle,
) -> u32 {
    opcode_disasm_wrapper(is32, is64, cs_base, ip, instr_bytes, distbuf, disasm_style)
}
