#[macro_use]
extern crate static_assertions;

/// Terminology:
/// - Guest: the target being emulated
/// - GPA: Guest Physical Address, the guests physical address space
/// - GVA: Guest Virtual Address, the guests virtual address space translated
///   using the guest page tables and a particular cr3.
/// - HVA: Host Virtual Address, an address valid in the emulator itself, NOT
///   the guest
mod cpu;
mod hook;
mod instr;
mod mem;
mod log;

pub use crate::cpu::*;
pub use crate::hook::*;
pub use crate::instr::*;
pub use crate::mem::*;
pub use crate::log::*;
