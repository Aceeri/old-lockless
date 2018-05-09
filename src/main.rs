
extern crate lockless;
extern crate rayon;

use std::sync::Arc;
use std::error::Error;

fn main() -> Result<(), String> {
    let mut application = lockless::Application::new_client("")?;
    application.run();

    Ok(())
}
