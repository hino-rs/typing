use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::event::{self, Event, KeyCode};
use std::io::Write;
use std::sync::mpsc;

fn main() -> std::io::Result<()> {
    let (tx, rx) = mpsc::channel();

    enable_raw_mode()?;

    std::thread::spawn(move || {
        loop {
            std::thread::sleep(std::time::Duration::from_millis(50));
            if let Ok(c) = rx.try_recv() {
                print!("{c}");
            } else {
                print!(".");
            }
            std::io::stdout().flush().unwrap();
        }
    });

    loop {
        let event = event::read()?;

        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Char('q') => break,
                KeyCode::Char(c) => {
                    tx.send(c).unwrap();
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    Ok(())
}
