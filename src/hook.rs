use std::ffi::c_void;
use std::mem;
use std::ptr;

use bochscpu::{Address, PhyAddress};
use bochscpu::hook::*;

struct FfiHooks {
    ctx: *mut c_void,

    reset: Option<extern "C" fn(*mut c_void, u32, u32)>,
    hlt: Option<extern "C" fn(*mut c_void, u32)>,
    mwait: Option<extern "C" fn(*mut c_void, u32, PhyAddress, usize, u32)>,
}

impl Default for FfiHooks {
    fn default() -> Self {
        Self { 
            ctx: ptr::null_mut(),
            reset: None,
            hlt: None,
            mwait: None,
        }
    }
}

impl Hooks for FfiHooks {
    fn reset(&mut self, id: u32, ty: u32) {
        self.reset.map(|f| f(self.ctx(), id, ty));
    }

    fn hlt(&mut self, id: u32) {
        self.hlt.map(|f| f(self.ctx(), id));
    }

    fn mwait(&mut self, id: u32, addr: PhyAddress, len: usize, flags: u32) {
        self.mwait.map(|f| f(self.ctx(), id, addr, len, flags));
    }
}

impl FfiHooks {
    fn from_raw(p: *mut c_void) -> Box<Self> {
        unsafe { Box::from_raw(p as _) }
    }

    fn ctx(&self) -> *mut c_void {
        self.ctx
    }

    fn set_ctx(&mut self, ctx: *mut c_void) {
        self.ctx = ctx;
    }

    fn set_reset(&mut self, f: Option<extern "C" fn(*mut c_void, u32, u32)>) {
        self.reset = f;
    }

    fn set_hlt(&mut self, f: Option<extern "C" fn(*mut c_void, u32)>) {
        self.hlt = f;
    }

    fn set_mwait(&mut self, f: Option<extern "C" fn(*mut c_void, u32, PhyAddress, usize, u32)>) {
        self.mwait = f;
    }
}

#[no_mangle]
pub extern "C" fn bochscpu_hook_new() -> *mut c_void {
    let b = Box::new(FfiHooks::default());
    Box::into_raw(b) as _
}

#[no_mangle]
pub extern "C" fn bochscpu_hook_delete(ffi_hooks: *mut c_void) {
    FfiHooks::from_raw(ffi_hooks);
    // implicit drop
}

#[no_mangle]
pub extern "C" fn bochscpu_hook_ctx(ffi_hooks: *mut c_void) -> *mut c_void {
    let ffi = FfiHooks::from_raw(ffi_hooks);

    let ctx = ffi.ctx();

    mem::forget(ffi);

    ctx
}

#[no_mangle]
pub extern "C" fn bochscpu_hook_set_ctx(ffi_hooks: *mut c_void, ctx: *mut c_void) {
    let mut ffi = FfiHooks::from_raw(ffi_hooks);

    ffi.set_ctx(ctx);

    mem::forget(ffi);
}

#[no_mangle]
pub extern "C" fn bochscpu_hook_reset(ffi_hooks: *mut c_void, hook: Option<extern "C" fn(*mut c_void, u32, u32)>) {
    let mut ffi = FfiHooks::from_raw(ffi_hooks);

    ffi.set_reset(hook);

    mem::forget(ffi);
}

#[no_mangle]
pub extern "C" fn bochscpu_hook_hlt(ffi_hooks: *mut c_void, hook: Option<extern "C" fn(*mut c_void, u32)>) {
    let mut ffi = FfiHooks::from_raw(ffi_hooks);

    ffi.set_hlt(hook);

    mem::forget(ffi);
}

#[no_mangle]
pub extern "C" fn bochscpu_hook_mwait(ffi_hooks: *mut c_void, hook: Option<extern "C" fn(*mut c_void, u32, PhyAddress, usize, u32)>) {
    let mut ffi = FfiHooks::from_raw(ffi_hooks);

    ffi.set_mwait(hook);

    mem::forget(ffi);
}