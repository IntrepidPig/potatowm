// Man fuck all this error handling

use std::io;
use std::error::Error;

use log::SetLoggerError;
use log::LogLevelFilter;
use log4rs::Handle;
use log4rs::init_config;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Logger, Root, Errors};

#[derive(Debug)]
pub enum LoggerInitError {
	IoError,
	ConfigError,
	LoggerError,
}

impl From<io::Error> for LoggerInitError {
	fn from(e: io::Error) -> LoggerInitError {
		println!("{}", e.description());
		LoggerInitError::IoError
	}
}

impl From<Errors> for LoggerInitError {
	fn from(e: Errors) -> LoggerInitError {
		println!("{}", e.description());
		LoggerInitError::ConfigError
	}
}

impl From<SetLoggerError> for LoggerInitError {
	fn from(e: SetLoggerError) -> LoggerInitError {
		println!("{}", e.description());
		LoggerInitError::LoggerError
	}
}

pub fn init_logging() -> Result<Handle, LoggerInitError> {
	let stdout = ConsoleAppender::builder()
		.encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} {h([{l}])}: {m}{n}")))
		.build();

	let requests = try!(FileAppender::builder()
		.encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} [{l}]: {m}{n}")))
		.build("log/log.log"));
	
	let config = try!(Config::builder()
		.appender(Appender::builder().build("stdout", Box::new(stdout)))
		.appender(Appender::builder().build("requests", Box::new(requests)))
		.logger(Logger::builder().build("app::backend::db", LogLevelFilter::Info))
		.logger(Logger::builder()
			.appender("requests")
			.additive(false)
			.build("app::requests", LogLevelFilter::Info))
		.build(Root::builder().appender("stdout").build(LogLevelFilter::Info)));
	
	let handle = try!(init_config(config));

	Ok(handle)
}
