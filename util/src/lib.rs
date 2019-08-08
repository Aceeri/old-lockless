#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_async;

#[macro_use]
extern crate lazy_static;

pub use slog::*;
pub use logger::LOG;

pub mod logger;
