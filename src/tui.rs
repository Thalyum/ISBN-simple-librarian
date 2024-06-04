use anyhow::Result;
use crossterm::{
    event::{self, poll, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    prelude::*,
    style::Stylize,
    symbols::border,
    widgets::{
        block::{Position, Title},
        Block, Borders, Paragraph, Widget,
    },
    Terminal,
};
use std::{
    io::{stdout, Stdout},
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::library::Library;

pub fn restore() -> Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

pub type TuiTerm = Terminal<CrosstermBackend<Stdout>>;

pub fn init() -> Result<TuiTerm> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let term = Terminal::new(CrosstermBackend::new(stdout()))?;
    Ok(term)
}

pub struct App {
    library: Arc<Mutex<Library>>,
    exit: bool,
}

impl App {
    pub fn with_library(library: Arc<Mutex<Library>>) -> Self {
        App {
            library,
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut TuiTerm) -> Result<()> {
        terminal.clear()?;

        // Main loop
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    self.exit = true;
                }
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('a') {
                    self.library
                        .lock()
                        .unwrap()
                        .register_book("123456".to_string(), None)?;
                }
            }
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame<'_>) {
        frame.render_widget(self, frame.size());
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Counter App Tutorial ".bold());
        let instructions = Title::from(Line::from(vec![
            " Add 'test' book ".into(),
            "<a>".blue().bold(),
            " Quit ".into(),
            "<q> ".blue().bold(),
        ]));
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Books in library: ".into(),
            self.library
                .lock()
                .unwrap()
                .books
                .len()
                .to_string()
                .yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
