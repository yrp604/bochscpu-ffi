use std::ffi::c_void;
use std::mem;
use std::ptr;

use bochscpu::cpu::*;

use crate::hook::FfiHooks;

pub type FfiCpu = *mut c_void;

/// Create a new Cpu
///
/// Create a new Cpu with the specified id. If SMP is not enabled, the id is
/// ignored.
#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_new(id: u32) -> FfiCpu {
    let c = Box::new(Cpu::new(id));
    Box::into_raw(c) as _
}

/// Delete a cpu
#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_delete(p: FfiCpu) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.delete();
}

/// Start emulation
///
/// To hook emulation, pass in a NULL terminated list of pointers to FfiHook
/// structs.
#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_run(p: FfiCpu, h: *mut *mut FfiHooks) {
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

/// Get rax
#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_rax(p: FfiCpu) -> u64 {
    let c: Box<Cpu> = Box::from_raw(p as _);

    let r = c.rax();

    mem::forget(c);

    r
}

/// Set rax
#[no_mangle]
pub unsafe extern "C" fn bochscpu_cpu_set_rax(p: FfiCpu, val: u64) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.set_rax(val);

    mem::forget(c);
}
