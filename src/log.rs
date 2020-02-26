#[no_mangle]
pub extern "C" fn bochscpu_log_set_level(level: u32) {
    stderrlog::new().verbosity(level).init().unwrap();
}
