use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};


use std::{
    io::{self, Result},
    result,
};
mod tui;

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
        }
        Ok(())
    }

    pub fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    pub fn handle_events(&mut self) -> Result<()> {
        todo!()
    }
}


impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from("Counter App".bold());
        let instructions = Title::from(Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
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
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

fn main() -> Result<()> {
    let mut term = tui::init()?;
    term.clear();
    let app = App::default().run(&mut term);
    Ok(())
}
