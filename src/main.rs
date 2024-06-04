// external
use anyhow::Result;
use clap::Parser;
use std::{
    sync::{Arc, Mutex},
    thread,
};
use tiny_http::Server;

// internal
use http::Method;
use library::Library;
use tui::App;

mod book;
mod collection;
mod error;
mod http;
mod library;
mod tui;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// IP address to bind
    #[arg(short, long, required = true)]
    bind: String,

    /// Port to bind
    #[arg(short, long, default_value_t = 8080)]
    port: u32,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let sock_addr = format!("{}:{}", args.bind, args.port);
    let server = Server::http(sock_addr).unwrap();

    let server = Arc::new(server);
    let mut guards = Vec::with_capacity(4);

    let library = Arc::new(Mutex::new(Library::new()));

    for _ in 0..4 {
        let server = Arc::clone(&server);
        let mut library = Arc::clone(&library);

        let guard = thread::spawn(move || loop {
            let rq = server.recv().unwrap();

            let res = Method::try_from(rq.method()).map(|m| m.handle_request(rq, &mut library));
            if res.is_err() {
                println!("{:?}", res);
                continue;
            }
        });

        guards.push(guard);
    }

    let mut terminal = tui::init()?;
    App::with_library(library).run(&mut terminal)?;
    tui::restore()
}
