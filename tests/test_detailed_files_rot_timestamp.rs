use flexi_logger::{detailed_format, Cleanup, Logger};
use log::*;

#[test]
fn test_detailed_files_rot_timestamp() {
    let handle = Logger::with_str("info")
        .format(detailed_format)
        .log_to_file()
        .rotate(2000, Cleanup::Never)
        .o_timestamp(true)
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));

    error!("This is an error message");
    warn!("This is a warning");
    info!("This is an info message");
    debug!("This is a debug message - you must not see it!");
    trace!("This is a trace message - you must not see it!");
    handle.validate_logs(&[
        ("ERROR", "test_detailed_files_rot", "error"),
        ("WARN", "test_detailed_files_rot", "warning"),
        ("INFO", "test_detailed_files_rot", "info"),
    ]);
}
