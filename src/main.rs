extern crate pencil;

#[macro_use] extern crate log;
extern crate env_logger;

use pencil::{Pencil, Request, Response, PencilResult};
use std::env;
use std::str::FromStr;
use std::path::PathBuf;

fn hello(_: &mut Request) -> PencilResult {
    Ok(Response::from("Hello World!"))
}

fn listen_port() -> u16 {
    let port_str = env::var("PORT").unwrap_or(String::new());
    FromStr::from_str(&port_str).unwrap_or(5000)
}

fn static_path() -> PathBuf {
    env::current_dir().unwrap_or(env::home_dir().unwrap_or(env::temp_dir()))
}

fn running_on_heroku() -> bool {
    env::var("DYNO").is_ok()
}

fn main() {
    let listen_addr = if running_on_heroku() { "0.0.0.0" } else { "127.0.0.1" };
    let port = listen_port();
    let root_path = static_path().to_string_lossy().into_owned();
    let mut app = Pencil::new(&root_path);

    // Routes
    app.get("/", "hello", hello);

    // Logging
    app.set_debug(true);
    app.set_log_level();
    env_logger::init().unwrap();

    // Listen and run
    debug!("Listening at {}:{} (root: {}).", listen_addr, port, root_path);
    app.run((listen_addr.as_ref(), port));
}
