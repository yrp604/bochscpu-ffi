use std::ffi::c_void;

use bochscpu::hook::*;
use bochscpu::{Address, PhyAddress};

/// FFI Hook object
///
/// Full desciptions of hook points can be found here:
/// http://bochs.sourceforge.net/cgi-bin/lxr/source/instrument/instrumentation.txt
///
/// If the hook value is NULL it will be treated as a no-op. The value of the
/// ctx field will be passed as the first paramter to every hook and is fully
/// controlled by the API author
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[repr(C)]
pub struct FfiHooks {
    pub ctx: *mut c_void,

    pub reset: Option<extern "C" fn(*mut c_void, u32, u32)>,
    pub hlt: Option<extern "C" fn(*mut c_void, u32)>,
    pub mwait: Option<extern "C" fn(*mut c_void, u32, u64, usize, u32)>,

    pub cnear_branch_taken: Option<extern "C" fn(*mut c_void, u32, u64, u64)>,
    pub cnear_branch_not_taken: Option<extern "C" fn(*mut c_void, u32, u64)>,
    pub ucnear_branch: Option<extern "C" fn(*mut c_void, u32, u32, u64, u64)>,
    pub far_branch: Option<extern "C" fn(*mut c_void, u32, u32, u16, u64, u16, u64)>,

    pub opcode: Option<extern "C" fn(*mut c_void, u32, *mut c_void, *const u8, usize, bool, bool)>,
    pub interrupt: Option<extern "C" fn(*mut c_void, u32, u32)>,
    pub exception: Option<extern "C" fn(*mut c_void, u32, u32, u32)>,
    pub hw_interrupt: Option<extern "C" fn(*mut c_void, u32, u32, u16, u64)>,

    pub tlb_cntrl: Option<extern "C" fn(*mut c_void, u32, u32, u64)>,
    pub cache_cntrl: Option<extern "C" fn(*mut c_void, u32, u32)>,
    pub prefetch_hint: Option<extern "C" fn(*mut c_void, u32, u32, u32, u64)>,
    pub clflush: Option<extern "C" fn(*mut c_void, u32, u64, u64)>,

    pub before_execution: Option<extern "C" fn(*mut c_void, u32, *mut c_void)>,
    pub after_execution: Option<extern "C" fn(*mut c_void, u32, *mut c_void)>,
    pub repeat_execution: Option<extern "C" fn(*mut c_void, u32, *mut c_void)>,

    pub inp: Option<extern "C" fn(*mut c_void, u16, usize)>,
    pub inp2: Option<extern "C" fn(*mut c_void, u16, usize)>,
    pub outp: Option<extern "C" fn(*mut c_void, u16, usize)>,

    pub lin_access: Option<extern "C" fn(*mut c_void, u32, u64, u64, usize, u32, u32)>,
    pub phy_access: Option<extern "C" fn(*mut c_void, u32, u64, usize, u32, u32)>,

    pub wrmsr: Option<extern "C" fn(*mut c_void, u32, u32, u64)>,

    pub vmexit: Option<extern "C" fn(*mut c_void, u32, u32, u64)>,
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
        self.cnear_branch_taken
            .map(|f| f(self.ctx, id, branch_pc, new_pc));
    }

    fn cnear_branch_not_taken(&mut self, id: u32, pc: Address) {
        self.cnear_branch_not_taken.map(|f| f(self.ctx, id, pc));
    }

    fn ucnear_branch(&mut self, id: u32, what: Branch, branch_pc: Address, new_pc: Address) {
        self.ucnear_branch
            .map(|f| f(self.ctx, id, what as u32, branch_pc, new_pc));
    }

    fn far_branch(
        &mut self,
        id: u32,
        what: Branch,
        branch_pc: (u16, Address),
        new_pc: (u16, Address),
    ) {
        self.far_branch.map(|f| {
            f(
                self.ctx,
                id,
                what as u32,
                branch_pc.0,
                branch_pc.1,
                new_pc.0,
                new_pc.1,
            )
        });
    }
}
