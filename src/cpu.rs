use std::ffi::c_void;
use std::mem::{self, ManuallyDrop};
use std::ptr;

use bochscpu::cpu::*;

use crate::hook::bochscpu_hooks_t;

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
#[allow(non_camel_case_types)]
pub type bochscpu_cpu_float80_t = Float80;

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

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_mode(p: bochscpu_cpu_t) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.set_mode();
}

/// Start emulation
///
/// To hook emulation, pass in a NULL terminated list of one or more pointers to
/// bochscpu_hooks_t structs.
#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_run(p: bochscpu_cpu_t, h: *mut *mut bochscpu_hooks_t) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

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
}

/// Stop emulation
///
#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_stop(p: bochscpu_cpu_t) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.set_run_state(RunState::Stop);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_state(p: bochscpu_cpu_t, s: *mut bochscpu_cpu_state_t) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    *s = c.state();
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_state(p: bochscpu_cpu_t, s: *const bochscpu_cpu_state_t) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.set_state(&*s)
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_state_no_flush(p: bochscpu_cpu_t, s: *const bochscpu_cpu_state_t) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.set_state_no_flush(&*s)
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_exception(p: bochscpu_cpu_t, vector: u32, error: u16) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.set_exception(vector, Some(error))
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_rax(p: bochscpu_cpu_t) -> u64 {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.rax()
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_rax(p: bochscpu_cpu_t, val: u64) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.set_rax(val)
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_rcx(p: bochscpu_cpu_t) -> u64 {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.rcx()
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_rcx(p: bochscpu_cpu_t, val: u64) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.set_rcx(val)
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_rdx(p: bochscpu_cpu_t) -> u64 {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.rdx()
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_rdx(p: bochscpu_cpu_t, val: u64) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.set_rdx(val)
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_rbx(p: bochscpu_cpu_t) -> u64 {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.rbx()
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_rbx(p: bochscpu_cpu_t, val: u64) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.set_rbx(val)
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_rsp(p: bochscpu_cpu_t) -> u64 {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.rsp()
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_rsp(p: bochscpu_cpu_t, val: u64) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.set_rsp(val)
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_rbp(p: bochscpu_cpu_t) -> u64 {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.rbp()
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_rbp(p: bochscpu_cpu_t, val: u64) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.set_rbp(val);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_rsi(p: bochscpu_cpu_t) -> u64 {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.rsi()
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_rsi(p: bochscpu_cpu_t, val: u64) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.set_rsi(val)
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_rdi(p: bochscpu_cpu_t) -> u64 {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.rdi()
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_rdi(p: bochscpu_cpu_t, val: u64) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.set_rdi(val)
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_r8(p: bochscpu_cpu_t) -> u64 {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.r8()
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_r8(p: bochscpu_cpu_t, val: u64) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.set_r8(val)
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_r9(p: bochscpu_cpu_t) -> u64 {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.r9()
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_r9(p: bochscpu_cpu_t, val: u64) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.set_r9(val)
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_r10(p: bochscpu_cpu_t) -> u64 {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.r10()
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_r10(p: bochscpu_cpu_t, val: u64) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.set_r10(val)
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_r11(p: bochscpu_cpu_t) -> u64 {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.r11()
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_r11(p: bochscpu_cpu_t, val: u64) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.set_r11(val)
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_r12(p: bochscpu_cpu_t) -> u64 {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.r12()
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_r12(p: bochscpu_cpu_t, val: u64) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.set_r12(val)
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_r13(p: bochscpu_cpu_t) -> u64 {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.r13()
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_r13(p: bochscpu_cpu_t, val: u64) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.set_r13(val);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_r14(p: bochscpu_cpu_t) -> u64 {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.r14()
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_r14(p: bochscpu_cpu_t, val: u64) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.set_r14(val);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_r15(p: bochscpu_cpu_t) -> u64 {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.r15()
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_r15(p: bochscpu_cpu_t, val: u64) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.set_r15(val);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_rip(p: bochscpu_cpu_t) -> u64 {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.rip()
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_rip(p: bochscpu_cpu_t, val: u64) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.set_rip(val);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_rflags(p: bochscpu_cpu_t) -> u64 {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.rflags()
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_rflags(p: bochscpu_cpu_t, val: u64) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.set_rflags(val);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_es(p: bochscpu_cpu_t, s: *mut bochscpu_cpu_seg_t) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    *s = c.es();
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_es(p: bochscpu_cpu_t, s: *const bochscpu_cpu_seg_t) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.set_es(*s);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_cs(p: bochscpu_cpu_t, s: *mut bochscpu_cpu_seg_t) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    *s = c.cs();
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_cs(p: bochscpu_cpu_t, s: *const bochscpu_cpu_seg_t) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.set_cs(*s);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_ss(p: bochscpu_cpu_t, s: *mut bochscpu_cpu_seg_t) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    *s = c.ss();
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_ss(p: bochscpu_cpu_t, s: *const bochscpu_cpu_seg_t) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.set_ss(*s);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_ds(p: bochscpu_cpu_t, s: *mut bochscpu_cpu_seg_t) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    *s = c.ds();
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_ds(p: bochscpu_cpu_t, s: *const bochscpu_cpu_seg_t) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.set_ds(*s);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_fs(p: bochscpu_cpu_t, s: *mut bochscpu_cpu_seg_t) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    *s = c.fs();
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_fs(p: bochscpu_cpu_t, s: *const bochscpu_cpu_seg_t) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.set_fs(*s);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_gs(p: bochscpu_cpu_t, s: *mut bochscpu_cpu_seg_t) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    *s = c.gs();
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_gs(p: bochscpu_cpu_t, s: *const bochscpu_cpu_seg_t) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.set_gs(*s);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_ldtr(p: bochscpu_cpu_t, s: *mut bochscpu_cpu_seg_t) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    *s = c.ldtr();
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_ldtr(p: bochscpu_cpu_t, s: *const bochscpu_cpu_seg_t) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.set_ldtr(*s);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_tr(p: bochscpu_cpu_t, s: *mut bochscpu_cpu_seg_t) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    *s = c.tr();
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_tr(p: bochscpu_cpu_t, s: *const bochscpu_cpu_seg_t) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.set_tr(*s);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_gdtr(p: bochscpu_cpu_t, s: *mut bochscpu_cpu_global_seg_t) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    *s = c.gdtr();
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_gdtr(p: bochscpu_cpu_t, s: *const bochscpu_cpu_global_seg_t) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.set_gdtr(*s);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_idtr(p: bochscpu_cpu_t, s: *mut bochscpu_cpu_global_seg_t) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    *s = c.idtr();
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_idtr(p: bochscpu_cpu_t, s: *const bochscpu_cpu_global_seg_t) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.set_idtr(*s);
}

// TODO crX/drX

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_cr2(p: bochscpu_cpu_t) -> u64 {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.cr2()
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_cr2(p: bochscpu_cpu_t, val: u64) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.set_cr2(val)
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_cr3(p: bochscpu_cpu_t) -> u64 {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.cr3()
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_cr3(p: bochscpu_cpu_t, val: u64) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.set_cr3(val)
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_zmm(p: bochscpu_cpu_t, idx: usize, z: *mut bochscpu_cpu_zmm_t) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    *z = c.zmm(idx);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_zmm(
    p: bochscpu_cpu_t,
    idx: usize,
    z: *const bochscpu_cpu_zmm_t,
) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.set_zmm(idx, *z)
}

// TODO fp/msr
#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_fp_st(p: bochscpu_cpu_t, idx: usize, f: *mut bochscpu_cpu_float80_t) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    *f = c.fp_st(idx);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_fp_st(
    p: bochscpu_cpu_t,
    idx: usize,
    f: *const bochscpu_cpu_float80_t,
) {
    let c: ManuallyDrop<Box<Cpu>> = ManuallyDrop::new(Box::from_raw(p as _));

    c.set_fp_st(idx, *f)
}
