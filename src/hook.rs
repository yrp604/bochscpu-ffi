use std::ffi::c_void;
use std::mem;
use std::ptr;

use bochscpu::{Address, PhyAddress};
use bochscpu::hook::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[repr(C)]
struct FfiHooks {
    pub ctx: *mut c_void,

    pub reset: Option<extern "C" fn(*mut c_void, u32, u32)>,
    pub hlt: Option<extern "C" fn(*mut c_void, u32)>,
    pub mwait: Option<extern "C" fn(*mut c_void, u32, u64, usize, u32)>,

    pub cnear_branch_taken: Option<extern "C" fn(*mut c_void, u32, u64, u64)>,
    pub cnear_branch_not_taken: Option<extern "C" fn(*mut c_void, u32, u64)>,
    pub ucnear_branch: Option<extern "C" fn(*mut c_void, u32, u32, u64, u64)>,
    pub far_branch: Option<extern "C" fn(*mut c_void, u32, u32, u16, u64, u16, u64)>,
}

impl Default for FfiHooks {
    fn default() -> Self {
        Self {
            ctx: ptr::null_mut(),
            reset: None,
            hlt: None,
            mwait: None,

            cnear_branch_taken: None,
            cnear_branch_not_taken: None,
            ucnear_branch: None,
            far_branch: None,
        }
    }
}

impl Hooks for FfiHooks {
    fn reset(&mut self, id: u32, ty: u32) {
        self.reset.map(|f| f(self.ctx, id, ty));
    }

    fn hlt(&mut self, id: u32) {
        self.hlt.map(|f| f(self.ctx, id));
    }

    fn mwait(&mut self, id: u32, addr: PhyAddress, len: usize, flags: u32) {
        self.mwait.map(|f| f(self.ctx, id, addr, len, flags));
    }

     fn cnear_branch_taken(&mut self, id: u32, branch_pc: Address, new_pc: Address) {
        self.cnear_branch_taken.map(|f| f(self.ctx, id, branch_pc, new_pc));
     }

     fn cnear_branch_not_taken(&mut self, id: u32, pc: Address) {
        self.cnear_branch_not_taken.map(|f| f(self.ctx, id, pc));
     }

     fn ucnear_branch(&mut self, id: u32, what: Branch, branch_pc: Address, new_pc: Address) {
        self.ucnear_branch.map(|f| f(self.ctx, id, what as u32, branch_pc, new_pc));
     }

     fn far_branch(&mut self, id: u32, what: Branch, branch_pc: (u16, Address), new_pc: (u16, Address)) {
        self.far_branch.map(|f| f(self.ctx, id, what as u32, branch_pc.0, branch_pc.1, new_pc.0, new_pc.1));
     }
}

#[no_mangle]
pub extern "C" fn bochscpu_hook_new() -> *mut c_void {
    let b = Box::new(FfiHooks::default());
    Box::into_raw(b) as _
}

#[no_mangle]
pub extern "C" fn bochscpu_hook_delete(p: *mut c_void) {
    let ffi: Box<FfiHooks> = unsafe { Box::from_raw(p as _) };

    mem::drop(ffi);
}

#[no_mangle]
pub extern "C" fn bochscpu_hook_ctx(p: *mut c_void) -> *mut c_void {
    let ffi: Box<FfiHooks> = unsafe { Box::from_raw(p as _) };

    let ctx = ffi.ctx;

    mem::forget(ffi);

    ctx
}

#[no_mangle]
pub extern "C" fn bochscpu_hook_set_ctx(p: *mut c_void, ctx: *mut c_void) {
    let mut ffi: Box<FfiHooks> = unsafe { Box::from_raw(p as _) };

    ffi.ctx = ctx;

    mem::forget(ffi);
}

#[no_mangle]
pub extern "C" fn bochscpu_hook_reset(p: *mut c_void, hook: Option<extern "C" fn(*mut c_void, u32, u32)>) {
    let mut ffi: Box<FfiHooks> = unsafe { Box::from_raw(p as _) };

    ffi.reset = hook;

    mem::forget(ffi);
}

#[no_mangle]
pub extern "C" fn bochscpu_hook_hlt(p: *mut c_void, hook: Option<extern "C" fn(*mut c_void, u32)>) {
    let mut ffi: Box<FfiHooks> = unsafe { Box::from_raw(p as _) };

    ffi.hlt = hook;

    mem::forget(ffi);
}

#[no_mangle]
pub extern "C" fn bochscpu_hook_mwait(p: *mut c_void, hook: Option<extern "C" fn(*mut c_void, u32, PhyAddress, usize, u32)>) {
    let mut ffi: Box<FfiHooks> = unsafe { Box::from_raw(p as _) };

    ffi.mwait = hook;

    mem::forget(ffi);
}

#[no_mangle]
pub extern "C" fn bochscpu_hook_cnear_branch_taken(p: *mut c_void, hook: Option<extern "C" fn(*mut c_void, u32, u64, u64)>) {
    let mut ffi: Box<FfiHooks> = unsafe { Box::from_raw(p as _) };

    ffi.cnear_branch_taken = hook;

    mem::forget(ffi);
}

#[no_mangle]
pub extern "C" fn bochscpu_hook_cnear_branch_not_taken(p: *mut c_void, hook: Option<extern "C" fn(*mut c_void, u32, u64)>) {
    let mut ffi: Box<FfiHooks> = unsafe { Box::from_raw(p as _) };

    ffi.cnear_branch_not_taken = hook;

    mem::forget(ffi);
}

#[no_mangle]
pub extern "C" fn bochscpu_hook_ucnear_branch(p: *mut c_void, hook: Option<extern "C" fn(*mut c_void, u32, u32, u64, u64)>) {
    let mut ffi: Box<FfiHooks> = unsafe { Box::from_raw(p as _) };

    ffi.ucnear_branch = hook;

    mem::forget(ffi);
}

#[no_mangle]
pub extern "C" fn bochscpu_hook_far_branch(p: *mut c_void, hook: Option<extern "C" fn(*mut c_void, u32, u32, u16, u64, u16, u64)>) {
    let mut ffi: Box<FfiHooks> = unsafe { Box::from_raw(p as _) };

    ffi.far_branch = hook;

    mem::forget(ffi);
}
