
use slog::Drain;

lazy_static! {
    pub static ref LOG: slog::Logger = { setup() };
}

pub fn setup() -> slog::Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let log = slog::Logger::root(drain, o!());
    info!(log, "Logger set up");
    log
}
