use std::ffi::c_void;

use bochscpu::hook::*;

/// These calls are all 1:1 mappings to the hooks listed here:
/// http://bochs.sourceforge.net/cgi-bin/lxr/source/instrument/instrumentation.txt

pub unsafe extern "C" fn bochscpu_hook_after_execution(h: extern "C" fn(id: u32, ins: *mut c_void)) {
    after_execution(move |id, v| h(id, v));
}

pub unsafe extern "C" fn bochscpu_hook_after_execution_clear() {
    after_execution_clear()
}

pub unsafe extern "C" fn bochscpu_hook_before_execution(h: extern "C" fn(id: u32, ins: *mut c_void)) {
    before_execution(move |id, v| h(id, v));
}

pub unsafe extern "C" fn bochscpu_hook_before_execution_clear() {
    before_execution_clear();
}

pub unsafe extern "C" fn bochscpu_hook_cache_cntrl(h: extern "C" fn(id: u32, what: u32)) {
    cache_cntrl(move |id, c| h(id, c as u32));
}

pub unsafe extern "C" fn bochscpu_hook_cache_cntrl_clear() {
    cache_cntrl_clear();
}

pub unsafe extern "C" fn bochscpu_hook_clfush(h: extern "C" fn(id: u32, gva: u64, gpa: u64)) {
    clflush(move |id, gva, gpa| h(id, gva, gpa))
}

pub unsafe extern "C" fn bochscpu_hook_clflush_clear() {
    clflush_clear()
}

pub unsafe extern "C" fn bochscpu_hook_cnear_branch_not_taken(h: extern "C" fn(id: u32, gva: u64)) {
    cnear_branch_not_taken(move |id, gva| h(id, gva))
}

pub unsafe extern "C" fn bochscpu_hook_cnear_branch_not_taken_clear() {
    cnear_branch_not_taken_clear()
}

pub unsafe extern "C" fn bochscpu_hook_cnear_branch_taken(h: extern "C" fn(id: u32, old: u64, new: u64)) {
    cnear_branch_taken(move |id, old, new| h(id, old, new))
}

pub unsafe extern "C" fn bochscpu_hook_cnear_branch_taken_clear() {
    cnear_branch_taken_clear()
}

pub unsafe extern "C" fn bochscpu_hook_exception(h: extern "C" fn(id: u32, vector: u32, error_code: u32)) {
    exception(move |id, vector, error_code| h(id, vector, error_code));
}

pub unsafe extern "C" fn bochscpu_hook_exception_clear() {
    exception_clear()
}

pub unsafe extern "C" fn bochscpu_hook_far_branch(h: extern "C" fn(id: u32, what: u32, prev_cs: u16, prev_pc: u64, new_cs: u16, new_pc: u64)) {
    far_branch(move |id, what, prev: (u16, u64), new: (u16, u64)| h(id, what as u32, prev.0, prev.1, new.0, new.1));
}

pub unsafe extern "C" fn bochscpu_hook_far_branch_clear() {
    far_branch_clear()
}

pub unsafe extern"C" fn bochscpu_hook_clear() {
    clear()
}
