use log::LevelFilter;
use simplelog::{TermLogger, TerminalMode, Config, ColorChoice};

pub fn init(level: u8) {
  let log_level = match level {
    0 => {LevelFilter::Error},
    1 => {LevelFilter::Info},
    _ => {LevelFilter::Debug},
  };

  TermLogger::init(
    log_level,
    Config::default(),
    TerminalMode::Stdout,
    ColorChoice::Auto
  ).unwrap();
}