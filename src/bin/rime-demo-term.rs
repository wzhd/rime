use std::fs;
use std::io::Write;
use std::os::raw::c_int;

use rime::Context;
use rime::RimeRs;
use rustbox::Key;
use rustbox::{Color, InitOptions, RustBox};
use unicode_width::UnicodeWidthChar;

fn main() {
    run();
}

fn run() {
    let term = TermPrinter::new();

    let rime = RimeRs::new().expect("Failed to start rime");
    let session = rime.create_session();

    let mut commited = String::new();

    loop {
        let context = session.context().expect("Failed to get rime context");
        let composing = context.composition.preedit.is_some();
        match term.poll().expect("rustbox.poll_event error") {
            rustbox::Event::KeyEvent(key) => match key {
                Key::Ctrl('c') => break,
                Key::Enter => {
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
                _ => {}
            },
            _ => {}
        }
        let context = session.context().expect("Failed to get rime context");
        term.rustbox.clear();
        if let Some(commit) = session.get_commit().expect("Error with get_commit") {
            commited.push_str(&commit.text);
        }
        term.print_text(0, 0, "Recent output: ");
        let mut recent: Vec<&str> = commited.lines().rev().take(3).collect();
        recent.reverse();

        for (l, y) in recent.iter().zip(0..) {
            term.print_text(16, y, l);
        }
        show_rime_menu(&context, &term);
        term.rustbox.present();
    }
    let mut out_file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("rimers-output.txt")
        .expect("Can't write to file");
    out_file
        .write_all(commited.as_bytes())
        .expect("Can't save typed text.");
    ()
}

fn show_rime_menu(context: &Context, term: &TermPrinter) {
    term.print_text(0, 3, "Preedit: ");
    if let Some(ref preedit) = context.composition.preedit {
        term.print_text(9, 3, preedit);
    }
    term.print_text(0, 4, "Preview: ");
    if let Some(ref preview) = context.commit_text_preview {
        term.print_text(9, 4, preview);
    }
    term.print_text(0, 5, "Candidates: ");
    for (candidate, i) in context.menu.candidates.iter().zip(1..) {
        let num = format!("{}.", i);
        term.print_text(4, 5 + i, &num);
        term.print_text(6, 5 + i, &candidate.text);
    }
}

struct TermPrinter {
    pub rustbox: RustBox,
}

impl TermPrinter {
    fn new() -> TermPrinter {
        let mut options: InitOptions = Default::default();
        options.buffer_stderr = true;
        let rustbox = RustBox::init(options).expect("RustBox::init error.");

        rustbox.print(
            1,
            1,
            rustbox::RB_BOLD,
            Color::White,
            Color::Black,
            "Hello, world!",
        );
        rustbox.print(
            1,
            3,
            rustbox::RB_BOLD,
            Color::White,
            Color::Black,
            "Press 'q' to quit.",
        );
        rustbox.present();
        TermPrinter { rustbox }
    }
    /// Avoid overlapping
    fn print_text(&self, x: usize, y: usize, text: &str) {
        let width = self.rustbox.width();
        let mut pos = x;
        for c in text.chars() {
            if pos > width {
                break;
            }
            let cw = c.width().unwrap_or(1);
            self.rustbox
                .print_char(pos, y, rustbox::RB_BOLD, Color::Black, Color::White, c);
            pos += cw;
        }
    }
    fn poll(&self) -> rustbox::EventResult {
        self.rustbox.poll_event(false)
    }
}
