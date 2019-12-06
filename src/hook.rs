use std::ffi::c_void;

use bochscpu::hook::*;

/// These calls are all 1:1 mappings to the hooks listed here:
/// http://bochs.sourceforge.net/cgi-bin/lxr/source/instrument/instrumentation.txt

#[no_mangle]
pub unsafe extern "C" fn bochscpu_hook_after_execution(h: extern "C" fn(id: u32, ins: *mut c_void)) {
    after_execution(move |id, v| h(id, v));
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_hook_after_execution_clear() {
    after_execution_clear()
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_hook_before_execution(h: extern "C" fn(id: u32, ins: *mut c_void)) {
    before_execution(move |id, v| h(id, v));
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_hook_before_execution_clear() {
    before_execution_clear();
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_hook_cache_cntrl(h: extern "C" fn(id: u32, what: u32)) {
    cache_cntrl(move |id, c| h(id, c as u32));
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_hook_cache_cntrl_clear() {
    cache_cntrl_clear();
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_hook_clfush(h: extern "C" fn(id: u32, gva: u64, gpa: u64)) {
    clflush(move |id, gva, gpa| h(id, gva, gpa))
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_hook_clflush_clear() {
    clflush_clear()
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_hook_cnear_branch_not_taken(h: extern "C" fn(id: u32, gva: u64)) {
    cnear_branch_not_taken(move |id, gva| h(id, gva))
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_hook_cnear_branch_not_taken_clear() {
    cnear_branch_not_taken_clear()
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_hook_cnear_branch_taken(h: extern "C" fn(id: u32, old: u64, new: u64)) {
    cnear_branch_taken(move |id, old, new| h(id, old, new))
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_hook_cnear_branch_taken_clear() {
    cnear_branch_taken_clear()
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_hook_exception(h: extern "C" fn(id: u32, vector: u32, error_code: u32)) {
    exception(move |id, vector, error_code| h(id, vector, error_code));
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_hook_exception_clear() {
    exception_clear()
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_hook_far_branch(h: extern "C" fn(id: u32, what: u32, prev_cs: u16, prev_pc: u64, new_cs: u16, new_pc: u64)) {
    far_branch(move |id, what, prev: (u16, u64), new: (u16, u64)| h(id, what as u32, prev.0, prev.1, new.0, new.1));
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_hook_far_branch_clear() {
    far_branch_clear()
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_hook_hlt(h: extern "C" fn(id: u32)) {
    hlt(move |id| h(id))
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_hook_hlt_clear() {
    hlt_clear()
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_hook_hw_interrupt(h: extern "C" fn(id: u32, vector: u32, cs: u16, pc: u64)) {
    hw_interrupt(move |id, vector, cspc: (u16, u64)| h(id, vector, cspc.0, cspc.1))
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_hook_hw_interrupt_clear() {
    hw_interrupt_clear()
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_hook_inp(h: extern "C" fn(addr: u16, len: usize)) {
    inp(move |addr, len| h(addr, len))
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_hook_inp_clear() {
    inp_clear()
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_hook_inp2(h: extern "C" fn(addr: u16, len: usize, val: u32)) {
    inp2(move |addr, len, val| h(addr, len, val))
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_hook_inp2_clear() {
    inp2_clear()
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_hook_interrupt(h: extern "C" fn(id: u32, vector: u32)) {
    interrupt(move |id, vector| h(id, vector))
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_hook_interrupt_clear() {
    interrupt_clear()
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_hook_lin_access(h: extern "C" fn(id: u32, gva: u64, gpa: u64, sz: usize, ty: u32, rw: u32)) {
    lin_access(move |id, gva, gpa, sz, ty, rw| h(id, gva, gpa, sz, ty, rw as u32))
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_hook_lin_access_clear() {
    lin_access_clear()
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_hook_mwait(h: extern "C" fn(id: u32, gpa: u64, len: usize, flags: u32)) {
    mwait(move |id, gpa, len, flags| h(id, gpa, len, flags as u32))
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_hook_mwait_clear() {
    mwait_clear()
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_hook_opcode(h: extern "C" fn(id: u32, i: *mut c_void, opcode: *const u8, usize, u32, u32)) {
    opcode(move |id, i, s: &[u8], is32, is64| h(id, i, s.as_ptr(), s.len(), is32 as u32, is64 as u32))
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_hook_opcode_clear() {
    opcode_clear()
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_hook_outp(h: extern "C" fn(addr: u16, len: usize, val: u32)) {
    outp(move |addr, len, val| h(addr, len, val))
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_hook_outp_clear() {
    outp_clear()
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_hook_phy_access(h: extern "C" fn(id: u32, gpa: u64, sz: usize, ty: u32, rw: u32)) {
    phy_access(move |id, gpa, sz, ty, rw| h(id, gpa, sz, ty, rw as u32))
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_hook_phy_access_clear() {
    phy_access_clear()
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_hook_prefetch_hint(h: extern "C" fn(id: u32, what: u32, seg: u32, off: u64)) {
    prefetch_hint(move |id, what, seg, off| h(id, what as u32, seg, off))
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_hook_prefetch_hint_clear() {
    prefetch_hint_clear()
}

#[no_mangle]
pub unsafe extern"C" fn bochscpu_hook_clear() {
    clear()
}
