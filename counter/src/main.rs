use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*,*}
};
use std::{io::{self, Result},result};
use crossterm::event::{self,Event,KeyCode,KeyEvent,KeyEventKind};
mod tui;

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
}

impl App {
    pub fn run(&mut self,terminal: &mut tui::Tui) -> Result<()>{
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
        }
        Ok(())
    }

    pub fn render_frame(&self, frame: &mut Frame){
        todo!()
    }

    pub fn handle_events(&mut self)->Result<()>{
        todo!()
    }
}

fn main () -> Result<()> {
    let mut term = tui::init()?;
    let app = App::default().run(&mut term);
    Ok(())
}