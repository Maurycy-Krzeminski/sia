use crossterm::{
    event::{self},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use log::LevelFilter;
use ratatui::{
    prelude::{CrosstermBackend, Terminal},
    widgets::{Block, Borders, List, Paragraph}, style::{Style, Color},
};
use core::fmt;
use std::io::{stdout, Result, Error};
use sysinfo::System;


#[derive(Debug)]
struct MyError;


impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Custom error occurred")
    }
}

impl std::error::Error for MyError {}


impl From<MyError> for Error {
    fn from(err: MyError) -> Error {
        Error::new(std::io::ErrorKind::Unsupported, err)
    }
}


fn main() -> Result<()> {


    let mut is_modal_open = false;
    let log_path = "app_log.log";

    // Open the file with options to append if it exists or create if it doesn't
    let log_file= std::fs::OpenOptions::new()
        .create(true) // Create the file if it doesn't exist
        .append(true) // Append to the file if it exists
        .open(log_path)
        .expect("Failed to open file");

    // Initialize logger
    env_logger::Builder::from_default_env()
        .filter_level(LevelFilter::Info)
        .format_timestamp(None) // Optionally format the timestamp
        .write_style(env_logger::WriteStyle::Always) // Ensure logs are written to the file even without a TTY
        .target(env_logger::Target::Pipe(Box::new(log_file))) // Redirect logs to the file
        .init();

    println!("Hello, world!");
    // Please note that we use "new_all" to ensure that all list of
    // components, network interfaces, disks and users are already
    // filled!
    let mut sys = System::new_all();
    if sysinfo::IS_SUPPORTED_SYSTEM {
        println!("supported system")
    }
    else {
        println!("Not supported os");
        return  Err(MyError.into());
    }

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    loop {
        // First we update all information of our `System` struct.
        sys.refresh_all();
        // RAM and swap information:
        let total_memory =  format!("total memory: {} bytes", sys.total_memory());
        let used_memory =  format!("used memory : {} bytes", sys.used_memory());
        let total_swap =  format!("total swap  : {} bytes", sys.total_swap());
        let used_swap =  format!("used swap   : {} bytes", sys.used_swap());
        // Display system information:
        let system_name =  format!("System name:             {:?}", System::name().unwrap_or("Unknown".to_string()));
        let kernel_version =  format!("System kernel version:   {:?}", System::kernel_version().unwrap_or("Unknown".to_string()));
        let os_version =  format!("System OS version:       {:?}", System::os_version().unwrap_or("Unknown".to_string()));
        let host_name =  format!("System host name:        {:?}", System::host_name().unwrap_or("Unknown".to_string()));
        // Number of CPUs:
        let nb_cpus = format!("Number of CPUs: {}", sys.cpus().len());
        let cpus: Vec<_> = sys.cpus().into_iter().map(|cpu| format!("cpu {:?}, {:?} , {:?}, {:?}  ", cpu.name(), cpu.frequency(), cpu.brand(), cpu.vendor_id())).collect();

        let items = [
            total_memory,
            used_memory,
            total_swap,
            used_swap,
            system_name,
            kernel_version,
            os_version,
            host_name,
            nb_cpus,
        ];

        let items_vec: Vec<_> = items.iter().cloned().collect();

        let result: Vec<_> = items_vec.into_iter().chain(cpus.into_iter()).collect();
        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                //                Paragraph::new(total_memory + + &used_memory)
                List::new(result)
                    .style(Style::default().fg(Color::Yellow))
                    .block(
                        Block::new().borders(Borders::ALL).title("Block Title")
                    ),
                area,
            );
        })?;
        if is_modal_open {
            // Render a simple modal-like overlay
            terminal.draw(|frame| {
                let area = frame.size();
                frame.render_widget(
                    Paragraph::new("modal open"),
                    area,
                );
            })?;
        } else {
            // Render your main content here
        }
        if event::poll(std::time::Duration::from_millis(16))? {
            // If a key event occurs, handle it
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if key.kind == crossterm::event::KeyEventKind::Press {
                    match key.code {
                        crossterm::event::KeyCode::Char('?') => {
                            log::info!("test question mark");
                            is_modal_open = true;
                        },
                        crossterm::event::KeyCode::Char('q') => break,
                        _ => {},
                    }
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())

}
