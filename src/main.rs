
extern crate lockless;

fn main() {
    let mut application = lockless::Application::new_client("");
    application.run();
}
