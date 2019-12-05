/// Terminology:
/// - Guest: the target being emulated
/// - GPA: Guest Physical Address, the guests physical address space
/// - GVA: Guest Virtual Address, the guests virtual address space translated
///   using the guest page tables and a particular cr3.
/// - HVA: Host Virtual Address, an address valid in the emulator itself, NOT
///   the guest

mod hook;
mod mem;

pub use hook::*;
pub use mem::*;
