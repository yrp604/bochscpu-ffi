#[unsafe(no_mangle)]
pub extern "C" fn bochscpu_log_set_level(level: usize) {
    stderrlog::new().verbosity(level).init().unwrap();
}
