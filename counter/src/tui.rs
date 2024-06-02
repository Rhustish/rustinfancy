use std::io::{stdout,Stdout,Result};

use crossterm::{execute,terminal::*};
use ratatui::prelude::*;


//ye bas type h, line 10 k result wale field m use karne ke liye :|
pub type Tui = Terminal<CrosstermBackend<Stdout>>;

pub fn init() -> Result<Tui> {
    execute!(stdout(),EnterAlternateScreen)?;
    enable_raw_mode()?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

pub fn restore() -> Result<()> {
    execute!(stdout(),LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}