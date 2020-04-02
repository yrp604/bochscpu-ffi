use std::ffi::c_void;

use bochscpu::hook::*;
use bochscpu::{Address, PhyAddress};

// these have to be hard coded otherwise cbindgen wont pick up on them
pub const BOCHSCPU_HOOK_MEM_READ: u32 = 0;
const_assert_eq!(BOCHSCPU_HOOK_MEM_READ, MemAccess::Read as u32);

pub const BOCHSCPU_HOOK_MEM_WRITE: u32 = 1;
const_assert_eq!(BOCHSCPU_HOOK_MEM_WRITE, MemAccess::Write as u32);

pub const BOCHSCPU_HOOK_MEM_EXECUTE: u32 = 2;
const_assert_eq!(BOCHSCPU_HOOK_MEM_EXECUTE, MemAccess::Execute as u32);

pub const BOCHSCPU_HOOK_MEM_RW: u32 = 3;
const_assert_eq!(BOCHSCPU_HOOK_MEM_RW, MemAccess::RW as u32);

pub const BOCHSCPU_HOOK_TLB_CR0: u32 = 10;
const_assert_eq!(BOCHSCPU_HOOK_TLB_CR0, TlbCntrl::MovCr0 as u32);

pub const BOCHSCPU_HOOK_TLB_CR3: u32 = 11;
const_assert_eq!(BOCHSCPU_HOOK_TLB_CR3, TlbCntrl::MovCr3 as u32);

pub const BOCHSCPU_HOOK_TLB_CR4: u32 = 12;
const_assert_eq!(BOCHSCPU_HOOK_TLB_CR4, TlbCntrl::MovCr4 as u32);

pub const BOCHSCPU_HOOK_TLB_TASKSWITCH: u32 = 13;
const_assert_eq!(BOCHSCPU_HOOK_TLB_TASKSWITCH, TlbCntrl::TaskSwitch as u32);

pub const BOCHSCPU_HOOK_TLB_CONTEXTSWITCH: u32 = 14;
const_assert_eq!(BOCHSCPU_HOOK_TLB_CONTEXTSWITCH, TlbCntrl::ContextSwitch as u32);

pub const BOCHSCPU_HOOK_TLB_INVLPG: u32 = 15;
const_assert_eq!(BOCHSCPU_HOOK_TLB_INVLPG, TlbCntrl::InvLpg as u32);

pub const BOCHSCPU_HOOK_TLB_INVEPT: u32 = 16;
const_assert_eq!(BOCHSCPU_HOOK_TLB_INVEPT, TlbCntrl::InvEpt as u32);

pub const BOCHSCPU_HOOK_TLB_INVVPID: u32 = 17;
const_assert_eq!(BOCHSCPU_HOOK_TLB_INVVPID, TlbCntrl::InvVpid as u32);

pub const BOCHSCPU_HOOK_TLB_INVPCID: u32 = 18;
const_assert_eq!(BOCHSCPU_HOOK_TLB_INVPCID, TlbCntrl::InvPcid as u32);

/// FFI Hook object
///
/// Full desciptions of hook points can be found here:
/// http://bochs.sourceforge.net/cgi-bin/lxr/source/instrument/instrumentation.txt
///
/// If the hook value is NULL it will be treated as a no-op. The value of the
/// ctx field will be passed as the first paramter to every hook and is fully
/// controlled by the API author
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[repr(C)]
pub struct bochscpu_hooks_t {
    pub ctx: *mut c_void,

    pub reset: Option<extern "C" fn(*mut c_void, u32, u32)>,
    pub hlt: Option<extern "C" fn(*mut c_void, u32)>,
    pub mwait: Option<extern "C" fn(*mut c_void, u32, u64, usize, u32)>,

    pub cnear_branch_taken: Option<extern "C" fn(*mut c_void, u32, u64, u64)>,
    pub cnear_branch_not_taken: Option<extern "C" fn(*mut c_void, u32, u64, u64)>,
    pub ucnear_branch: Option<extern "C" fn(*mut c_void, u32, u32, u64, u64)>,
    pub far_branch: Option<extern "C" fn(*mut c_void, u32, u32, u16, u64, u16, u64)>,

    pub opcode: Option<extern "C" fn(*mut c_void, u32, *const c_void, *const u8, usize, bool, bool)>,
    pub interrupt: Option<extern "C" fn(*mut c_void, u32, u32)>,
    pub exception: Option<extern "C" fn(*mut c_void, u32, u32, u32)>,
    pub hw_interrupt: Option<extern "C" fn(*mut c_void, u32, u32, u16, u64)>,

    pub tlb_cntrl: Option<extern "C" fn(*mut c_void, u32, u32, u64)>,
    pub cache_cntrl: Option<extern "C" fn(*mut c_void, u32, u32)>,
    pub prefetch_hint: Option<extern "C" fn(*mut c_void, u32, u32, u32, u64)>,
    pub clflush: Option<extern "C" fn(*mut c_void, u32, u64, u64)>,

    pub before_execution: Option<extern "C" fn(*mut c_void, u32, *mut c_void)>,
    pub after_execution: Option<extern "C" fn(*mut c_void, u32, *mut c_void)>,
    pub repeat_iteration: Option<extern "C" fn(*mut c_void, u32, *mut c_void)>,

    pub inp: Option<extern "C" fn(*mut c_void, u16, usize)>,
    pub inp2: Option<extern "C" fn(*mut c_void, u16, usize, u32)>,
    pub outp: Option<extern "C" fn(*mut c_void, u16, usize, u32)>,

    pub lin_access: Option<extern "C" fn(*mut c_void, u32, u64, u64, usize, u32, u32)>,
    pub phy_access: Option<extern "C" fn(*mut c_void, u32, u64, usize, u32, u32)>,

    pub wrmsr: Option<extern "C" fn(*mut c_void, u32, u32, u64)>,

    pub vmexit: Option<extern "C" fn(*mut c_void, u32, u32, u64)>,
}

impl Hooks for bochscpu_hooks_t {
    fn reset(&mut self, id: u32, ty: ResetSource) {
        self.reset.map(|f| f(self.ctx, id, ty as u32));
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

    fn cnear_branch_not_taken(&mut self, id: u32, pc: Address, new_pc: Address) {
        self.cnear_branch_not_taken.map(|f| f(self.ctx, id, pc, new_pc));
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

    fn opcode(&mut self, id: u32, ins: *mut c_void, opcode: &[u8], is_32: bool, is_64: bool) {
        self.opcode.map(|f| {
            f(
                self.ctx,
                id,
                ins,
                opcode.as_ptr(),
                opcode.len(),
                is_32,
                is_64,
            )
        });
    }

    fn interrupt(&mut self, id: u32, vector: u32) {
        self.interrupt.map(|f| f(self.ctx, id, vector));
    }

    fn exception(&mut self, id: u32, vector: u32, error_code: u32) {
        self.exception.map(|f| f(self.ctx, id, vector, error_code));
    }

    fn hw_interrupt(&mut self, id: u32, vector: u32, pc: (u16, Address)) {
        self.hw_interrupt
            .map(|f| f(self.ctx, id, vector, pc.0, pc.1));
    }

    fn tlb_cntrl(&mut self, id: u32, what: TlbCntrl, new_cr: Option<PhyAddress>) {
        let cr = match new_cr {
            Some(v) => v,
            None => 0,
        };
        self.tlb_cntrl.map(|f| f(self.ctx, id, what as u32, cr));
    }

    fn cache_cntrl(&mut self, id: u32, what: CacheCntrl) {
        self.cache_cntrl.map(|f| f(self.ctx, id, what as u32));
    }

    fn prefetch_hint(&mut self, id: u32, what: PrefetchHint, seg: u32, off: Address) {
        self.prefetch_hint
            .map(|f| f(self.ctx, id, what as u32, seg, off));
    }

    fn clflush(&mut self, id: u32, vaddr: Address, paddr: PhyAddress) {
        self.clflush.map(|f| f(self.ctx, id, vaddr, paddr));
    }

    fn before_execution(&mut self, id: u32, ins: *mut c_void) {
        self.before_execution.map(|f| f(self.ctx, id, ins));
    }

    fn after_execution(&mut self, id: u32, ins: *mut c_void) {
        self.after_execution.map(|f| f(self.ctx, id, ins));
    }

    fn repeat_iteration(&mut self, id: u32, ins: *mut c_void) {
        self.repeat_iteration.map(|f| f(self.ctx, id, ins));
    }

    fn inp(&mut self, addr: u16, len: usize) {
        self.inp.map(|f| f(self.ctx, addr, len));
    }

    fn inp2(&mut self, addr: u16, len: usize, val: u32) {
        self.inp2.map(|f| f(self.ctx, addr, len, val));
    }

    fn outp(&mut self, addr: u16, len: usize, val: u32) {
        self.outp.map(|f| f(self.ctx, addr, len, val));
    }

    fn lin_access(
        &mut self,
        id: u32,
        vaddr: Address,
        paddr: Address,
        len: usize,
        memty: MemType,
        rw: MemAccess,
    ) {
        self.lin_access
            .map(|f| f(self.ctx, id, vaddr, paddr, len, memty as u32, rw as u32));
    }

    fn phy_access(
        &mut self,
        id: u32,
        paddr: PhyAddress,
        len: usize,
        memty: MemType,
        rw: MemAccess,
    ) {
        self.phy_access
            .map(|f| f(self.ctx, id, paddr, len, memty as u32, rw as u32));
    }

    fn wrmsr(&mut self, id: u32, msr: u32, val: u64) {
        self.wrmsr.map(|f| f(self.ctx, id, msr, val));
    }

    fn vmexit(&mut self, id: u32, reason: u32, qualification: u64) {
        self.vmexit.map(|f| f(self.ctx, id, reason, qualification));
    }
}
