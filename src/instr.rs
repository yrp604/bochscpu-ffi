use std::ffi::c_void;

use bochscpu::opcode::*;

#[allow(non_camel_case_types)]
pub type bochscpu_instr_t = *const c_void;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bochscpu_instr_bx_opcode(p: bochscpu_instr_t) -> u32 {
    unsafe { instr_bx_opcode(p) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bochscpu_instr_imm16(p: bochscpu_instr_t) -> u16 {
    unsafe { instr_imm16(p) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bochscpu_instr_imm32(p: bochscpu_instr_t) -> u32 {
    unsafe { instr_imm32(p) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bochscpu_instr_imm64(p: bochscpu_instr_t) -> u64 {
    unsafe { instr_imm64(p) }
}
