use std::ffi::c_void;
use std::mem;
use std::ptr;

use bochscpu::cpu::*;

use crate::hook::bochscpu_ffi_hooks_t;

#[allow(non_camel_case_types)]
pub type bochscpu_cpu_t = *mut c_void;
#[allow(non_camel_case_types)]
pub type bochscpu_cpu_state_t = State;
#[allow(non_camel_case_types)]
pub type bochscpu_cpu_seg_t = Seg;
#[allow(non_camel_case_types)]
pub type bochscpu_cpu_global_seg_t = GlobalSeg;
#[allow(non_camel_case_types)]
pub type bochscpu_cpu_zmm_t = Zmm;

/// Create a new Cpu
///
/// Create a new Cpu with the specified id. If SMP is not enabled, the id is
/// ignored.
#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_new(id: u32) -> bochscpu_cpu_t {
    let c = Box::new(Cpu::new(id));
    Box::into_raw(c) as _
}

/// Create a new Cpu
///
/// Instantiate an already existing cpu with the specified id.
#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_from(id: u32) -> bochscpu_cpu_t {
    let c = Box::new(Cpu::from(id));
    Box::into_raw(c) as _
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_forget(p: bochscpu_cpu_t) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    mem::drop(c);
}

/// Delete a cpu
#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_delete(p: bochscpu_cpu_t) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.delete();
}

/// Start emulation
///
/// To hook emulation, pass in a NULL terminated list of one or more pointers to
/// bochscpu_ffi_hooks_t structs.
#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_run(p: bochscpu_cpu_t, h: *mut *mut bochscpu_ffi_hooks_t) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    let mut prep = c.prepare();

    if h != ptr::null_mut() {
        let mut ii = h;

        loop {
            if *ii == ptr::null_mut() {
                break;
            }

            prep = prep.register(&mut **ii);
            ii = ii.add(1);
        }
    }

    prep.run();

    mem::forget(c);
}

/// Stop emulation
///
#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_stop(p: bochscpu_cpu_t) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_run_state(RunState::Stop);

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_state(p: bochscpu_cpu_t, s: *mut bochscpu_cpu_state_t) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    *s = c.state();

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_state(p: bochscpu_cpu_t, s: *const bochscpu_cpu_state_t) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_state(&*s);

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_rax(p: bochscpu_cpu_t) -> u64 {
    let c: Box<Cpu> = Box::from_raw(p as _);

    let r = c.rax();

    mem::forget(c);

    r
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_rax(p: bochscpu_cpu_t, val: u64) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_rax(val);

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_rcx(p: bochscpu_cpu_t) -> u64 {
    let c: Box<Cpu> = Box::from_raw(p as _);

    let r = c.rcx();

    mem::forget(c);

    r
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_rcx(p: bochscpu_cpu_t, val: u64) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_rcx(val);

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_rdx(p: bochscpu_cpu_t) -> u64 {
    let c: Box<Cpu> = Box::from_raw(p as _);

    let r = c.rdx();

    mem::forget(c);

    r
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_rdx(p: bochscpu_cpu_t, val: u64) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_rdx(val);

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_rbx(p: bochscpu_cpu_t) -> u64 {
    let c: Box<Cpu> = Box::from_raw(p as _);

    let r = c.rbx();

    mem::forget(c);

    r
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_rbx(p: bochscpu_cpu_t, val: u64) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_rbx(val);

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_rsp(p: bochscpu_cpu_t) -> u64 {
    let c: Box<Cpu> = Box::from_raw(p as _);

    let r = c.rsp();

    mem::forget(c);

    r
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_rsp(p: bochscpu_cpu_t, val: u64) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_rsp(val);

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_rbp(p: bochscpu_cpu_t) -> u64 {
    let c: Box<Cpu> = Box::from_raw(p as _);

    let r = c.rbp();

    mem::forget(c);

    r
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_rbp(p: bochscpu_cpu_t, val: u64) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_rbp(val);

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_rsi(p: bochscpu_cpu_t) -> u64 {
    let c: Box<Cpu> = Box::from_raw(p as _);

    let r = c.rsi();

    mem::forget(c);

    r
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_rsi(p: bochscpu_cpu_t, val: u64) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_rsi(val);

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_rdi(p: bochscpu_cpu_t) -> u64 {
    let c: Box<Cpu> = Box::from_raw(p as _);

    let r = c.rdi();

    mem::forget(c);

    r
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_rdi(p: bochscpu_cpu_t, val: u64) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_rdi(val);

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_r8(p: bochscpu_cpu_t) -> u64 {
    let c: Box<Cpu> = Box::from_raw(p as _);

    let r = c.r8();

    mem::forget(c);

    r
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_r8(p: bochscpu_cpu_t, val: u64) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_r8(val);

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_r9(p: bochscpu_cpu_t) -> u64 {
    let c: Box<Cpu> = Box::from_raw(p as _);

    let r = c.r9();

    mem::forget(c);

    r
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_r9(p: bochscpu_cpu_t, val: u64) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_r9(val);

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_r10(p: bochscpu_cpu_t) -> u64 {
    let c: Box<Cpu> = Box::from_raw(p as _);

    let r = c.r10();

    mem::forget(c);

    r
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_r10(p: bochscpu_cpu_t, val: u64) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_r10(val);

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_r11(p: bochscpu_cpu_t) -> u64 {
    let c: Box<Cpu> = Box::from_raw(p as _);

    let r = c.r11();

    mem::forget(c);

    r
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_r11(p: bochscpu_cpu_t, val: u64) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_r11(val);

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_r12(p: bochscpu_cpu_t) -> u64 {
    let c: Box<Cpu> = Box::from_raw(p as _);

    let r = c.r12();

    mem::forget(c);

    r
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_r12(p: bochscpu_cpu_t, val: u64) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_r12(val);

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_r13(p: bochscpu_cpu_t) -> u64 {
    let c: Box<Cpu> = Box::from_raw(p as _);

    let r = c.r13();

    mem::forget(c);

    r
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_r13(p: bochscpu_cpu_t, val: u64) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_r13(val);

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_r14(p: bochscpu_cpu_t) -> u64 {
    let c: Box<Cpu> = Box::from_raw(p as _);

    let r = c.r14();

    mem::forget(c);

    r
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_r14(p: bochscpu_cpu_t, val: u64) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_r14(val);

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_r15(p: bochscpu_cpu_t) -> u64 {
    let c: Box<Cpu> = Box::from_raw(p as _);

    let r = c.r15();

    mem::forget(c);

    r
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_r15(p: bochscpu_cpu_t, val: u64) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_r15(val);

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_rip(p: bochscpu_cpu_t) -> u64 {
    let c: Box<Cpu> = Box::from_raw(p as _);

    let r = c.rip();

    mem::forget(c);

    r
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_rip(p: bochscpu_cpu_t, val: u64) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_rip(val);

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_rflags(p: bochscpu_cpu_t) -> u64 {
    let c: Box<Cpu> = Box::from_raw(p as _);

    let r = c.rflags();

    mem::forget(c);

    r
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_rflags(p: bochscpu_cpu_t, val: u64) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_rflags(val);

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_es(p: bochscpu_cpu_t, s: *mut bochscpu_cpu_seg_t) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    *s = c.es();

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_es(p: bochscpu_cpu_t, s: *const bochscpu_cpu_seg_t) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_es(*s);

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_cs(p: bochscpu_cpu_t, s: *mut bochscpu_cpu_seg_t) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    *s = c.cs();

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_cs(p: bochscpu_cpu_t, s: *const bochscpu_cpu_seg_t) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_cs(*s);

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_ss(p: bochscpu_cpu_t, s: *mut bochscpu_cpu_seg_t) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    *s = c.ss();

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_ss(p: bochscpu_cpu_t, s: *const bochscpu_cpu_seg_t) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_ss(*s);

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_ds(p: bochscpu_cpu_t, s: *mut bochscpu_cpu_seg_t) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    *s = c.ds();

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_ds(p: bochscpu_cpu_t, s: *const bochscpu_cpu_seg_t) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_ds(*s);

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_fs(p: bochscpu_cpu_t, s: *mut bochscpu_cpu_seg_t) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    *s = c.fs();

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_fs(p: bochscpu_cpu_t, s: *const bochscpu_cpu_seg_t) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_fs(*s);

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_gs(p: bochscpu_cpu_t, s: *mut bochscpu_cpu_seg_t) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    *s = c.gs();

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_gs(p: bochscpu_cpu_t, s: *const bochscpu_cpu_seg_t) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_gs(*s);

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_ldtr(p: bochscpu_cpu_t, s: *mut bochscpu_cpu_seg_t) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    *s = c.ldtr();

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_ldtr(p: bochscpu_cpu_t, s: *const bochscpu_cpu_seg_t) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_ldtr(*s);

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_tr(p: bochscpu_cpu_t, s: *mut bochscpu_cpu_seg_t) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    *s = c.tr();

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_tr(p: bochscpu_cpu_t, s: *const bochscpu_cpu_seg_t) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_tr(*s);

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_gdtr(p: bochscpu_cpu_t, s: *mut bochscpu_cpu_global_seg_t) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    *s = c.gdtr();

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_gdtr(p: bochscpu_cpu_t, s: *const bochscpu_cpu_global_seg_t) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_gdtr(*s);

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_idtr(p: bochscpu_cpu_t, s: *mut bochscpu_cpu_global_seg_t) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    *s = c.idtr();

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_idtr(p: bochscpu_cpu_t, s: *const bochscpu_cpu_global_seg_t) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_idtr(*s);

    mem::forget(c);
}

// TODO crX/drX

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_zmm(p: bochscpu_cpu_t, idx: usize, z: *mut bochscpu_cpu_zmm_t) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    *z = c.zmm(idx);

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_zmm(
    p: bochscpu_cpu_t,
    idx: usize,
    z: *const bochscpu_cpu_zmm_t,
) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_zmm(idx, *z);

    mem::forget(c);
}

// TODO fp/msr
