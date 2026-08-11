extern crate alloc;

use alloc::format;
use alloc::string::String;

use log::{Record, Level, Metadata};

pub fn app_log(msg: &str) {
    let mut c_str = String::with_capacity(msg.len() + 1);
    c_str.push_str(msg);
    c_str.push('\0');

    app_log_c_str(&c_str);
}

pub fn app_log_c_str(msg: &str) {
    unsafe {
        crate::ffi::sys::vm_app_log(msg.as_ptr());
    }
}

struct MreLogger;

impl log::Log for MreLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let level_str = match record.level() {
                Level::Error => "ERROR",
                Level::Warn  => "WARN ",
                Level::Info  => "INFO ",
                Level::Debug => "DEBUG",
                Level::Trace => "TRACE",
            };

            let file = record.file().unwrap_or("unknown");
            let line = record.line().unwrap_or(0);

            let message = format!("[{}] {}:{} - {}\0", level_str, file, line, record.args());

            app_log_c_str(&message);
        }
    }

    fn flush(&self) {}
}

static LOGGER: MreLogger = MreLogger;

pub fn init(max_level: log::LevelFilter) {
    unsafe {
        let _ = log::set_logger_racy(&LOGGER);
        log::set_max_level_racy(max_level);
    }
}