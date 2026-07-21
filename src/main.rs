use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::event::{self, Event, KeyCode};
use std::collections::VecDeque;
use std::io::Write;
use std::sync::mpsc;

fn main() -> std::io::Result<()> {
    let (input_tx, input_rx) = mpsc::channel();
    let (char_tx, char_rx) = mpsc::channel();

    enable_raw_mode()?;

    // 文字列生成スレッド
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(std::time::Duration::from_millis(250));
            let new_char = rand::random_range('a'..='z');
            if char_tx.send(new_char).is_err() {
                break; // 受信側が終了していたらループを抜ける
            }
        }
    });

    // 状態管理・描画スレッド
    std::thread::spawn(move || {
        let mut line = VecDeque::with_capacity(32);
        loop {
            std::thread::sleep(std::time::Duration::from_millis(50));
           
            if let Ok(new_char) = char_rx.try_recv() {
                line.push_back(new_char);
            }

            if let Ok(input) = input_rx.try_recv() {
                if let Some(front) = line.front() {
                    if &input == front {
                        line.pop_front();
                    }
                }
            }
            print!("\r{}", line.iter().collect::<String>());
            std::io::stdout().flush().unwrap();
        }
    });

    loop {
        let event = event::read()?;

        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Esc => break,
                KeyCode::Char(c) => {
                    input_tx.send(c).unwrap();
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    Ok(())
}
