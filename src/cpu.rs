use std::ffi::c_void;
use std::mem;
use std::ptr;

use bochscpu::cpu::{Cpu, GlobalSeg, Seg, State, Zmm};

use crate::hook::bochscpu_ffi_hooks;

#[allow(non_camel_case_types)]
pub type bochscpu_cpu = *mut c_void;
#[allow(non_camel_case_types)]
pub type bochscpu_cpu_state = State;
#[allow(non_camel_case_types)]
pub type bochscpu_cpu_seg = Seg;
#[allow(non_camel_case_types)]
pub type bochscpu_cpu_global_seg = GlobalSeg;
#[allow(non_camel_case_types)]
pub type bochscpu_cpu_zmm = Zmm;

/// Create a new Cpu
///
/// Create a new Cpu with the specified id. If SMP is not enabled, the id is
/// ignored.
#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_new(id: u32) -> bochscpu_cpu {
    let c = Box::new(Cpu::new(id));
    Box::into_raw(c) as _
}

/// Delete a cpu
#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_delete(p: bochscpu_cpu) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.delete();
}

/// Start emulation
///
/// To hook emulation, pass in a NULL terminated list of pointers to FfiHook
/// structs.
#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_run(p: bochscpu_cpu, h: *mut *mut bochscpu_ffi_hooks) {
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
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_state(p: bochscpu_cpu, s: *mut bochscpu_cpu_state) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    *s = c.state();

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_state(p: bochscpu_cpu, s: *const bochscpu_cpu_state) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_state(&*s);

    mem::forget(c);
}

/// Get rax
#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_rax(p: bochscpu_cpu) -> u64 {
    let c: Box<Cpu> = Box::from_raw(p as _);

    let r = c.rax();

    mem::forget(c);

    r
}

/// Set rax
#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_rax(p: bochscpu_cpu, val: u64) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_rax(val);

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_es(p: bochscpu_cpu, s: *mut bochscpu_cpu_seg) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    *s = c.es();

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_es(p: bochscpu_cpu, s: *const bochscpu_cpu_seg) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_es(*s);

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_cs(p: bochscpu_cpu, s: *mut bochscpu_cpu_seg) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    *s = c.cs();

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_cs(p: bochscpu_cpu, s: *const bochscpu_cpu_seg) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_cs(*s);

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_ss(p: bochscpu_cpu, s: *mut bochscpu_cpu_seg) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    *s = c.ss();

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_ss(p: bochscpu_cpu, s: *const bochscpu_cpu_seg) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_ss(*s);

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_ds(p: bochscpu_cpu, s: *mut bochscpu_cpu_seg) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    *s = c.ds();

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_ds(p: bochscpu_cpu, s: *const bochscpu_cpu_seg) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_ds(*s);

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_fs(p: bochscpu_cpu, s: *mut bochscpu_cpu_seg) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    *s = c.fs();

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_fs(p: bochscpu_cpu, s: *const bochscpu_cpu_seg) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_fs(*s);

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_gs(p: bochscpu_cpu, s: *mut bochscpu_cpu_seg) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    *s = c.gs();

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_gs(p: bochscpu_cpu, s: *const bochscpu_cpu_seg) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_gs(*s);

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_ldtr(p: bochscpu_cpu, s: *mut bochscpu_cpu_seg) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    *s = c.ldtr();

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_ldtr(p: bochscpu_cpu, s: *const bochscpu_cpu_seg) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_ldtr(*s);

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_tr(p: bochscpu_cpu, s: *mut bochscpu_cpu_seg) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    *s = c.tr();

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_tr(p: bochscpu_cpu, s: *const bochscpu_cpu_seg) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_tr(*s);

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_gdtr(p: bochscpu_cpu, s: *mut bochscpu_cpu_global_seg) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    *s = c.gdtr();

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_gdtr(p: bochscpu_cpu, s: *const bochscpu_cpu_global_seg) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_gdtr(*s);

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_idtr(p: bochscpu_cpu, s: *mut bochscpu_cpu_global_seg) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    *s = c.idtr();

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_idtr(p: bochscpu_cpu, s: *const bochscpu_cpu_global_seg) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_idtr(*s);

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_zmm(p: bochscpu_cpu, idx: usize, z: *mut bochscpu_cpu_zmm) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    *z = c.zmm(idx);

    mem::forget(c);
}

#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_zmm(p: bochscpu_cpu, idx: usize, z: *const bochscpu_cpu_zmm) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_zmm(idx, *z);

    mem::forget(c);
}
