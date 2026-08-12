extern crate alloc;

use alloc::format;
use alloc::string::String;

use log::{Record, Metadata, Level};

pub fn app_log(msg: &str) {
    if msg.ends_with('\0') {
        unsafe {
            crate::ffi::sys::vm_app_log(msg.as_ptr());
        }
    } else {
        let mut c_str = String::with_capacity(msg.len() + 1);
        c_str.push_str(msg);
        c_str.push('\0');
        
        unsafe {
            crate::ffi::sys::vm_app_log(c_str.as_ptr());
        }
    }
}

pub fn app_log_record(record: &Record) {
    let level_num = match record.level() {
        Level::Error => 2,
        Level::Warn  => 3,
        Level::Info  => 4,
        Level::Debug => 5,
        Level::Trace => 6, 
    };

    let date = crate::time::datetime::now().unwrap_or_default();
    let file = record.file().unwrap_or("unknown");
    let line = record.line().unwrap_or(0);

    let message = format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}\t{}\t{}:{}\t{}\0", 
            date.year, date.month, date.day, 
            date.hour, date.minute, date.second, 
            level_num, file, line, record.args());

    app_log(&message);
}

struct MreLogger;

impl log::Log for MreLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            app_log_record(record);
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