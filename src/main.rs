use routes::main::server;

pub mod handler;
pub mod routes;
pub mod utils;

fn main() {
    println!("Server Started");
    server();
}
