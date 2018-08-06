
extern crate lockless;
extern crate rayon;

fn main() -> Result<(), lockless::error::Error> {

    let mut application = lockless::Application::new_client("")?;
    application.run()?;

    Ok(())
}
