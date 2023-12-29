// $t@$h
use std::io::{self, Write};
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use crossterm::event::{self, Event, KeyCode, KeyEvent, poll};
use std::time::Duration;

fn print_splash() {
    println!("Welcome to QVLx Secboot Sim");
    println!("Architecture options:");
    println!("  x8664");
    println!("  arm64");
}

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    print_splash();

    let mut input = String::new();
    print!("secboot> ");
    io::stdout().flush().unwrap();

    while running.load(Ordering::SeqCst) {
        if poll(Duration::from_millis(500)).unwrap() {
            if let Ok(Event::Key(KeyEvent { code, .. })) = event::read() {
                match code {
                    KeyCode::Char(c) => {
                        print!("{}", c);
                        io::stdout().flush().unwrap();
                        input.push(c);
                    },
                    KeyCode::Backspace => {
                        if !input.is_empty() {
                            input.pop();
                            print!("\x08 \x08"); // Backspace, clear character, backspace again
                            io::stdout().flush().unwrap();
                        }
                    },
                    KeyCode::Enter => {
                        println!();
                        match input.trim() {
                            "x8664" => {
                                println!();
                                Command::new("x8664").spawn().expect("Failed to execute x8664");
                            },
                            "arm64" => {
                                println!();
                                Command::new("arm64").spawn().expect("Failed to execute arm64");
                            },
                            "exit" => {
                                println!("Exiting...");
                                break;
                            },
                            _ => {},
                        }
                        input.clear();
                        print!("secboot> ");
                        io::stdout().flush().unwrap();
                    },
                    _ => {}
                }
            }
        }
    }
}
