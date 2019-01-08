use std::os::raw::c_int;

use failure::Error;
use termion::color;
use termion::cursor::Goto;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use std::io::{stdin, stdout, Write};

use rime::Context;
use rime::RimeRs;
use termion::clear;

fn main() -> Result<(), Error> {
    run()
}

fn run() -> Result<(), Error> {
    let mut stdout = stdout().into_raw_mode()?;
    write!(
        stdout,
        "{}{}Welcome to Rime typer in the terminal!{}",
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

    let rime = RimeRs::new().expect("Failed to start rime");
    let session = rime.create_session();

    let mut commited = String::new();

    let stdin = stdin();
    for k in stdin.keys() {
        let context = session.context().expect("Failed to get rime context");
        let composing = context.composition.preedit.is_some();

        let k = k?;
        match k {
            Key::Ctrl('c') => break,
            Key::Char('\n') => {
                if composing {
                    session.process_key(0xff0d);
                } else {
                    commited += "\n";
                }
            }
            Key::Backspace => {
                if composing {
                    session.process_key(0xff08);
                } else {
                    commited.pop();
                }
            }
            Key::Char(' ') => {
                if composing {
                    session.process_key(b' ' as i32);
                } else {
                    commited += " ";
                }
            }
            Key::Char(c) => {
                session.process_key(c as c_int);
            }
            Key::F(n) => {
                // F1 = 0xffbe, F11 = 0xffc8, according to the source of fcitx
                let code = 0xffbd + n as i32;
                session.process_key(code);
            }
            _ => {}
        }

        let context = session.context()?;
        println!("{}\n", clear::All);
        if let Some(commit) = session.get_commit()? {
            commited.push_str(&commit.text);
        }
        print!("{}{}", Goto(1, 1), "Recent output: ");
        let mut recent: Vec<&str> = commited.lines().rev().take(3).collect();
        recent.reverse();

        for (l, y) in recent.iter().zip(1..) {
            print!("{}{}", Goto(17, y), l);
        }
        show_rime_menu(&context)?;
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
