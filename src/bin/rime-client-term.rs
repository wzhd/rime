use std::collections::VecDeque;
use std::env;
use std::io::{self, BufRead};
use std::net::{self, SocketAddr};
use std::os::raw::c_int;
use std::str::FromStr;

use failure::Error;
use termion::color;
use termion::cursor::Goto;
use termion::event::Key;
use termion::raw::IntoRawMode;

use std::io::{stdin, stdout, Write};

use rime::Context;
use rime::{KeyPress, Response};
use termion::clear;

fn main() -> Result<(), Error> {
    let mut args = env::args().skip(1);
    let addr = args.next().expect("Need server address as argument");
    let addr = SocketAddr::from_str(&addr).expect("Invalid address");
    run(addr)
}

fn run(addr: SocketAddr) -> Result<(), Error> {
    let mut stdout = stdout().into_raw_mode()?;
    write!(
        stdout,
        "{}{}Welcome to Rime client in the terminal!{}",
        termion::clear::All,
        Goto(1, 1),
        termion::cursor::Hide
    )?;
    write!(
        stdout,
        "{pos}{color}Use Ctrl-C to quit{reset}",
        pos = termion::cursor::Goto(1, 3),
        color = color::Fg(color::Blue),
        reset = color::Fg(color::Reset)
    )?;
    stdout.flush()?;
    let mut rime = RemoteRime::connect(addr)?;

    let stdin = stdin();
    // recently commited input
    let mut commited = VecDeque::new();

    use termion::input::TermRead;
    for k in stdin.keys() {
        let mut response: Option<Response> = None;
        let k = k?;
        match k {
            Key::Ctrl('c') => break,
            Key::Char('\n') => {
                response = Some(rime.process_key(0xff0d)?);
            }
            Key::Backspace => {
                response = Some(rime.process_key(0xff08)?);
            }
            Key::Char(c) => {
                response = Some(rime.process_key(c as c_int)?);
            }
            Key::F(n) => {
                // F1 = 0xffbe, F11 = 0xffc8, according to the source of fcitx
                let code = 0xffbd + n as i32;
                response = Some(rime.process_key(code)?);
            }
            _ => {}
        }
        if response.is_none() {
            continue;
        }
        let response = response.unwrap();

        println!("{}\n", clear::All);

        let context = &response.context;
        show_rime_menu(context)?;
        if let Some(commit) = response.commit {
            commited.push_back(commit.text);
            while commited.len() > 3 {
                commited.pop_front();
            }
        }
        print!("{}{}", Goto(1, 1), "Recent output: ");

        for (l, y) in commited.iter().zip(1..) {
            print!("{}{}", Goto(17, y), l);
        }
        stdout.flush()?;
    }
    Ok(())
}

fn show_rime_menu(context: &Context) -> Result<(), Error> {
    print!("{}{}", Goto(1, 4), "Preedit: ");
    if let Some(ref preedit) = context.composition.preedit {
        print!("{}{}", Goto(10, 4), preedit);
    }
    print!("{}{}", Goto(1, 5), "Preview: ");
    if let Some(ref preview) = context.commit_text_preview {
        print!("{}{}", Goto(10, 5), preview);
    }
    print!("{}{}", Goto(1, 6), "Candidates: ");
    for (candidate, i) in context.menu.candidates.iter().zip(1..) {
        print!("{}{}.", Goto(5, 6 + i), i);
        print!("{}{}", Goto(7, 6 + i), &candidate.text);
    }
    Ok(())
}

struct RemoteRime {
    stream: net::TcpStream,
    reader: io::BufReader<net::TcpStream>,
}

impl RemoteRime {
    fn connect(addr: SocketAddr) -> Result<RemoteRime, Error> {
        let stream = net::TcpStream::connect(addr)?;
        let reader = io::BufReader::new(stream.try_clone()?);
        Ok(RemoteRime { stream, reader })
    }

    fn process_key(&mut self, key: c_int) -> Result<Response, Error> {
        let e = KeyPress {
            key_code: key,
            mask: 0,
        };
        let j = serde_json::to_vec(&e)?;
        self.stream.write_all(&j)?;
        self.stream.write_all("\n".as_bytes())?;
        let mut line = String::new();
        self.reader.read_line(&mut line)?;
        let res = serde_json::from_str(&line)?;
        Ok(res)
    }
}
