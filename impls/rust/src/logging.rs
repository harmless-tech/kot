use crate::DEBUG_BUILD;
use log::{info, warn, LevelFilter};
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Config, Logger, Root},
    encode::pattern::PatternEncoder,
};
use std::fs::remove_file;

//TODO: Allow for log path to be changed?
static LOG_PATH: &str = "build/kot.log";

//TODO: Only allow logging when an argument is passed in to enable it.
pub fn setup_log() -> log4rs::Handle {
    // Cleanup
    let cleanup_log: bool = remove_file(LOG_PATH).is_ok();

    let filter = match DEBUG_BUILD {
        true => LevelFilter::Trace,
        false => LevelFilter::Info,
    };

    // Setup
    let stdout: ConsoleAppender = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{h({d(%H:%M:%S)(local)} - {l}: {m}{n})}",
        )))
        .build();

    let file_out: FileAppender = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{h({d(%H:%M:%S)(local)} - {l}: {m}{n})}",
        )))
        .build(LOG_PATH)
        .unwrap();

    let config: Config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("fileout", Box::new(file_out)))
        .logger(Logger::builder().build("app::backend::db", LevelFilter::Trace))
        .logger(
            Logger::builder()
                .appender("fileout")
                .additive(false)
                .build("app::fileout", filter),
        )
        .build(
            Root::builder()
                .appender("stdout")
                .appender("fileout")
                .build(filter),
        )
        .unwrap();

    let handle: log4rs::Handle = log4rs::init_config(config).unwrap();

    if cleanup_log {
        info!("Previous log file deleted.")
    }
    else {
        warn!("Previous log file could not be deleted. This could become a problem if the log file gets very long.")
    }

    handle
}
