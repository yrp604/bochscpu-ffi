use std::slice;

use bochscpu::mem::*;

/// Add GPA mapping to HVA
///
/// If the GPA was already mapped, this replaces the existing mapping
#[no_mangle]
pub unsafe extern "C" fn bochscpu_mem_page_insert(gpa: u64, hva: *mut u8) {
    page_insert(gpa, hva)
}

/// Delete GPA mapping
///
/// If the GPA is not valid, this is a no-op.
#[no_mangle]
pub unsafe extern "C" fn bochscpu_mem_page_remove(gpa: u64) {
    page_remove(gpa)
}

/// Install a physical page fault handler
///
/// This function will be called any time a request is made to physical memory
/// and the GPA is not present. This function should add a page using
/// `bochscpu_mem_page_insert()`
///
/// The paramter should have the type `void handler(gpa_t)`
///
/// This allows you to lazily page in your backing physical memory.
///
/// # Note
///
/// This is a global singleton, and installing a new physical page fault
/// handler will overwrite the existing handler.
#[no_mangle]
pub unsafe extern "C" fn bochscpu_mem_missing_page(handler: extern "C" fn(gpa: u64)) {
    missing_page(move |gpa| handler(gpa))
}

/// Translate GPA to HVA
///
/// # Panics
///
/// If the GPA does not exit, it will call the missing page handler. If no
/// missing page handler is set or the missing page handler does not add the
/// appropriate page, this will panic.
///
/// # Example
#[no_mangle]
pub unsafe extern "C" fn bochscpu_mem_phy_translate(gpa: u64) -> *mut u8 {
    phy_translate(gpa)
}

/// Translate GVA to GPA
///
/// Use the provided cr3 to translate the GVA into a GPA.
///
/// # Returns
///
/// Translated gpa on success, -1 on failure
#[no_mangle]
pub unsafe extern "C" fn bochscpu_mem_virt_translate(cr3: u64, gva: u64) -> u64 {
    match virt_translate_checked(cr3, gva) {
        Ok(a) => a,
        Err(_) => 0xffff_ffff_ffff_ffff,
    }
}

/// Read from GPA
///
/// # Panics
///
/// If the GPA does not exist, it will call the missing page function. If
/// that function does not exist or does not resolve the fault, this routine
/// will panic
#[no_mangle]
pub unsafe extern "C" fn bochscpu_mem_phy_read(gpa: u64, hva: *mut u8, sz: usize) {
    let s = slice::from_raw_parts_mut(hva, sz);
    phy_read_slice(gpa, s);
}

/// Write to GPA
///
/// # Panics
///
/// If the GPA does not exist, it will call the missing page function. If
/// that function does not exist or does not resolve the fault, this routine
/// will panic
#[no_mangle]
pub unsafe extern "C" fn bochscpu_mem_phy_write(gpa: u64, hva: *const u8, sz: usize) {
    let s = slice::from_raw_parts(hva, sz);
    phy_write(gpa, s);
}

/// Write to GVA
///
/// Write to GVA, using specified cr3 to translate.
///
/// # Returns
///
/// Zero on success, non-zero on failure
#[no_mangle]
pub unsafe extern "C" fn bochscpu_mem_virt_write(
    cr3: u64,
    gva: u64,
    hva: *const u8,
    sz: usize,
) -> i32 {
    let s = slice::from_raw_parts(hva, sz);

    match virt_write_checked(cr3, gva, s) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

/// Read from GVA
///
/// Read from GVA, using specified cr3 to translate.
///
/// # Returns
///
/// Zero on success, non-zero on failure
#[no_mangle]
pub unsafe extern "C" fn bochscpu_mem_virt_read(
    cr3: u64,
    gva: u64,
    hva: *mut u8,
    sz: usize,
) -> i32 {
    let s = slice::from_raw_parts_mut(hva, sz);

    match virt_read_slice_checked(cr3, gva, s) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}
