#[macro_use]
extern crate diesel;

mod domain;

mod server;

fn main() -> std::io::Result<()> {
    server::run()
}
