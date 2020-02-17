use std::ffi::c_void;
use std::mem;

use bochscpu::cpu::*;

pub unsafe extern "C" fn bochscpu_cpu_new(id: u32) -> *mut c_void {
    let c = Box::new(Cpu::new(id));
    Box::into_raw(c) as _
}

pub extern "C" fn bochscpu_cpu_delete(p: *mut c_void) {
    let c: Box<Cpu> = unsafe { Box::from_raw(p as _) };

    unsafe { c.delete() };

    mem::drop(c);
}
