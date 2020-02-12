use log::{Level, Metadata, Record};
use std::time::Instant;

pub struct Logger {
    level: Level,
    start_time: Instant,
}

impl Logger {
    pub fn init_with_level(level: Level) {
        let logger = Logger {
            level,
            start_time: Instant::now(),
        };
        log::set_boxed_logger(Box::new(logger)).unwrap();
        log::set_max_level(level.to_level_filter());
        log::debug!("Logger initialized");
    }

    pub fn short_level_name(level: Level) -> &'static str {
        return match level {
            Level::Error => "E",
            Level::Warn => "W",
            Level::Info => "I",
            Level::Debug => "D",
            Level::Trace => "T",
        };
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!(
                "[{} {}] {}",
                self.start_time.elapsed().as_millis(),
                Logger::short_level_name(record.level()),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}
