use std::ffi::{CStr, c_void};
use std::fs;
use std::mem;

use bochscpu::cpu::State;

#[no_mangle]
pub extern "C" fn bochscpu_axel_parse_state(p: *const i8) -> *mut State {
    let path = unsafe { CStr::from_ptr(p).to_str().unwrap() };
    let regs_data = fs::read_to_string(path).unwrap();

    let s: Box<State> = Box::new(serde_yaml::from_str(&regs_data).unwrap());

    Box::into_raw(s) as _
}

#[no_mangle]
pub extern "C" fn bochscpu_axel_free_state(p: *mut c_void) {
    let s: Box<State> = unsafe { Box::from_raw(p as _) };

    mem::drop(s);
}
