use failure::Error;
use rime::server::line_server::run;
use std::env;
use std::net::SocketAddr;
use std::str::FromStr;

fn main() -> Result<(), Error> {
    let arg = env::args().skip(1).next();
    let addr = arg.unwrap_or_else(|| "127.0.0.1:17878".into());
    let addr = SocketAddr::from_str(&addr).expect("Invalid address");
    run(addr)
}
