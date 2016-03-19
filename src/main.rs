extern crate pencil;

use pencil::{Pencil, Request, Response, PencilResult};
use std::env;
use std::str::FromStr;

fn hello(_: &mut Request) -> PencilResult {
    Ok(Response::from("Hello World!"))
}

fn listen_port() -> u16 {
    let port_str = env::var("PORT").unwrap_or(String::new());
    FromStr::from_str(&port_str).unwrap_or(5000)
}

fn main() {
    let port = listen_port();
    let listen_addr = format!("127.0.0.1:{}", port);
    println!("Listening at {}.", listen_addr);
    let mut app = Pencil::new("/web/hello");
    app.get("/", "hello", hello);
    app.run(("127.0.0.1", port));
}
