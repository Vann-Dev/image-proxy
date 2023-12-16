use routes::main::server;

pub mod handler;
pub mod routes;
pub mod utils;

fn main() {
    server();

    println!("Server Started");
}
