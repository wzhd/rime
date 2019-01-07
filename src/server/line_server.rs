/// Line-based protocol
use serde_derive::{Deserialize, Serialize};

use failure::Error;
use std::io::BufReader;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::Arc;

use super::threads::ThreadPool;
use crate::rimers::RimeRs;
use crate::KeyPress;
use std::io::BufRead;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: usize,
    y: i32,
}

pub fn run() -> Result<(), Error> {
    let listener = TcpListener::bind("127.0.0.1:17878")?;
    let pool = ThreadPool::new(4);
    let rime = Arc::new(RimeRs::new()?);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let rime = rime.clone();
        pool.execute(move || {
            if let Err(e) = handle_connection(stream, rime) {
                eprintln!("Error handling connection: {:?}", e);
            }
        });
    }
    Ok(())
}

fn handle_connection(connection: TcpStream, rime: Arc<RimeRs>) -> Result<(), Error> {
    let mut writer = connection.try_clone()?;
    let mut reader = BufReader::new(connection);
    let sess = { rime.create_session() };
    let mut line = String::new();
    loop {
        line.clear();
        let n = reader.read_line(&mut line)?;
        if n == 0 {
            return Ok(());
        } // EOF
        let k: KeyPress = serde_json::from_str(&line)?;
        let r = sess.process_press(k)?;
        let j = serde_json::to_vec(&r)?;
        writer.write_all(&j)?;
        writer.write_all("\n".as_bytes())?;
        writer.flush()?;
    }
}
