use rust_native::dev_server::{BuildStatus, DevServer, Platform};
use crossterm::{
    cursor, execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    event::{self, Event, KeyCode},
};
use std::io::{stdout, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut dev_server = DevServer::new();
    dev_server.watch(".")?;

    enable_raw_mode()?;
    loop {
        // Clear screen and draw UI
        execute!(
            stdout(),
            Clear(ClearType::All),
            cursor::MoveTo(0, 0),
            SetForegroundColor(Color::Green),
            Print("RustUI Development Server\n"),
            SetForegroundColor(Color::White),
        )?;

        // Display status
        let status = dev_server.get_status();
        print_status(&status)?;

        // Display controls
        println!("\nControls:");
        println!("r - Reload | s - Reopen | q - Quit");
        println!("1-4 - Switch Platform");

        // Handle events with proper error checking
        match event::read() {
            Ok(Event::Key(key_event)) => {
                match key_event.code {
                    KeyCode::Char('r') => dev_server.rebuild(),
                    KeyCode::Char('s') => {
                        dev_server = DevServer::new();
                        dev_server.watch(".")?;
                    }
                    KeyCode::Char('1') => dev_server.set_platform(Platform::Desktop),
                    KeyCode::Char('2') => dev_server.set_platform(Platform::IOS),
                    KeyCode::Char('3') => dev_server.set_platform(Platform::Android),
                    KeyCode::Char('4') => dev_server.set_platform(Platform::Web),
                    KeyCode::Char('q') => break,
                    _ => {}
                }
            }
            Err(e) => {
                return Err(Box::new(e));
            }
            _ => {}
        }
    }

    disable_raw_mode()?;
    Ok(())
}

fn print_status(status: &BuildStatus) -> Result<(), Box<dyn std::error::Error>> {
    let status_color = if status.in_progress {
        Color::Yellow
    } else if status.error.is_some() {
        Color::Red
    } else {
        Color::Green
    };

    execute!(
        stdout(),
        SetForegroundColor(status_color),
        Print(format!(
            "Status: {}\n",
            if status.in_progress {
                "Building..."
            } else if status.error.is_some() {
                "Error"
            } else {
                "Ready"
            }
        )),
    )?;

    if let Some(error) = &status.error {
        execute!(
            stdout(),
            SetForegroundColor(Color::Red),
            Print(format!("Error: {}\n", error)),
        )?;
    }

    Ok(())
}
