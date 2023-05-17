mod cli;
use clap::Parser;
use cli::Cli;

use crossterm::event::{DisableMouseCapture, EnableMouseCapture, KeyCode, Event, self};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use tui::backend::{CrosstermBackend, Backend};
use tui::layout::{Layout, Direction, Constraint, Alignment};
use tui::style::{Style, Color, Modifier};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph, Wrap};
use tui::{Terminal, Frame};

use std::error::Error;
use std::time::{Duration, Instant};
use std::{io};

struct App {
    scroll: u16,
    data: Vec<String>
}

impl App {
    fn new() -> Self {
        App {
            scroll: 0,
            data: Vec::new()
        }
    }

    fn on_tick(&mut self) {
        self.scroll += 1;
        self.scroll %= 10;
    }
}

fn run_terminal_tui() -> Result<(), Box<dyn Error>> {
    //TODO: Retrieve buffered lit of file
    // Compute how many lines can be printed in the terminal size as n (1 unit = char size)
    // And then read those n + k line as buffer smoothering
    let mut app = App::new();



    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(250);
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

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q'|'Q') = key.code {
                    return Ok(());
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();

    // Words made "loooong" to demonstrate line breaking.
    let s = "Veeeeeeeeeeeeeeeery    loooooooooooooooooong   striiiiiiiiiiiiiiiiiiiiiiiiiing.   ";
    let mut long_line = s.repeat(usize::from(size.width) / s.len() + 4);
    long_line.push('\n');

    let common_style = Style::default().bg(Color::Black).fg(Color::White);
    let block = Block::default().style(common_style);
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(100)
            ]
            .as_ref(),
        )
        .split(size);


    let mut text = vec![
        Spans::from("This is a line "),
        Spans::from(Span::styled(
            "This is a line   ",
            Style::default().fg(Color::Red).add_modifier(Modifier::SLOW_BLINK),
        )),
        Spans::from(Span::styled(
            "This is a line",
            Style::default().bg(Color::Blue),
        )),
        Spans::from(Span::styled(
            "This is a longer line",
            Style::default().add_modifier(Modifier::CROSSED_OUT),
        )),
        Spans::from(Span::styled(&long_line, Style::default().bg(Color::Green))),
        Spans::from(Span::styled(
            "This is a line",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::ITALIC),
        )),
    ];

    for line in app.data.iter() {
        text.push(Spans::from(line.as_str()))
    }

    let create_block = |title| {
        Block::default()
            .borders(Borders::ALL)
            .style(common_style)
            .title(Span::styled(
                title,
                Style::default().add_modifier(Modifier::BOLD),
            ))
    };

    let paragraph = Paragraph::new(text.clone())
        .style(Style::default().bg(Color::White).fg(Color::Black))
        .block(create_block("Left, no wrap"))
        .alignment(Alignment::Left);
    f.render_widget(paragraph, chunks[0]);

}

fn main() -> Result<(), Box<dyn Error>> {
    let cli: Cli = Cli::parse();

    // match cli.mode {
    //     cli::Mode::View => run_terminal_tui()?,
    //     cli::Mode::Split => cli.split_file()?
    // }

    cli.fake_test();
    Ok(())
}
