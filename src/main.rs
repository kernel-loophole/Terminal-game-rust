use crossterm::{
    cursor, ExecutableCommand, QueueableCommand, terminal, style, event::{self, KeyCode, KeyEvent, Event}
};
use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;

fn main() {
    // Initialize the terminal
    let mut stdout = stdout();
    stdout.execute(terminal::EnterAlternateScreen).unwrap();
    terminal::enable_raw_mode().unwrap();

    let mut x_pos = 0;
    let y_pos = 0;
    let width = 50; // Define the width of the movement area

    loop {
        // Non-blocking read of user input
        //testing using A and D KEY
        if event::poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
                match code {
                    KeyCode::Char('d') => {
                        // Move right
                        x_pos += 1;
                        if x_pos >= width {
                            x_pos = 0;
                        }
                    }
                    KeyCode::Char('a') => {
                        // Move left
                        if x_pos > 0 {
                            x_pos -= 1;
                        } else {
                            x_pos = width - 1;
                        }
                    }
                    KeyCode::Esc => {
                        break;
                    }
                    _ => {}
                }
            }
        }

        // Clear the terminal and update position
        stdout.queue(cursor::MoveTo(x_pos, y_pos)).unwrap();
        stdout.queue(terminal::Clear(terminal::ClearType::All)).unwrap();
        stdout.queue(cursor::MoveTo(x_pos, y_pos)).unwrap();
        stdout.queue(style::Print("O")).unwrap();

        // Flush the output to apply changes
        stdout.flush().unwrap();
    }

    terminal::disable_raw_mode().unwrap();
    stdout.execute(terminal::LeaveAlternateScreen).unwrap();
}
