use std::ffi::c_void;
use std::mem;

use bochscpu::cpu::*;

pub unsafe extern "C" fn bochscpu_cpu_new(id: u32) -> *mut c_void {
    let c = Box::new(Cpu::new(id));
    Box::into_raw(c) as _
}

pub unsafe extern "C" fn bochscpu_cpu_delete(p: *mut c_void) {
    let c: Box<Cpu> = Box::from_raw(p as _);

    c.delete();
}

pub unsafe extern "C" fn bochscpu_cpu_rax(p: *mut c_void) -> u64 {
    let c: Box<Cpu> = Box::from_raw(p as _);

    let r = c.rax();

    mem::forget(c);

    r
}

pub unsafe extern "C" fn bochscpu_cpu_set_rax(p: *mut c_void, val: u64) {
    let c: Box<Cpu> =  Box::from_raw(p as _);

    c.set_rax(val);

    mem::forget(c);
}
