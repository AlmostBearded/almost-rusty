use log::{Level, LevelFilter, Metadata, Record};

static mut MAX_LEVEL: Level = Level::Trace;

pub struct Logger;

impl Logger {
    pub fn activate() {
        log::set_logger(&Logger).unwrap();
        Logger::set_max_level(unsafe { MAX_LEVEL });
    }

    pub fn set_max_level(level: Level) {
        unsafe { MAX_LEVEL = level };
        log::set_max_level(match level {
            Level::Error => LevelFilter::Error,
            Level::Warn => LevelFilter::Warn,
            Level::Info => LevelFilter::Info,
            Level::Debug => LevelFilter::Debug,
            Level::Trace => LevelFilter::Trace,
        });
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= unsafe { MAX_LEVEL }
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}
