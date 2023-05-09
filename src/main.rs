mod cli;
use cli::Cli;
use clap::Parser;

use crossterm::event::{EnableMouseCapture, DisableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen};
use tui::Terminal;
use tui::backend::CrosstermBackend;

use std::time::Duration;
use std::{env, io};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::io::{BufWriter, Write};




struct App {
    scroll: u16,
}

impl App {
    fn new() -> App {
        App { scroll: 0 }
    }

    fn on_tick(&mut self) {
        self.scroll += 1;
        self.scroll %= 10;
    }
}

fn run_terminal_tui() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(250);
    let app = App::new();
    let res = run_app(&mut terminal, app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}
fn main() -> Result<(), Box<dyn Error>> {
    let cli: Cli = Cli::parse();

    if cli.is_split_mode() {
        run_terminal_tui();
    } else {
        cli.split_file()?;
    }


    Ok(())
}
