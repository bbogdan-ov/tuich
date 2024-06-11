mod app;
mod state;
mod widget;

use std::io;

use app::App;
use tuich::{backend::{crossterm::CrosstermBackend, BackendEvent}, terminal::Terminal, widget::RefDraw};

type Term = Terminal<CrosstermBackend<io::Stdout>>;

/// Main message
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Msg {
    #[default]
    None,
    Draw,
    Resize(u16, u16),
    Quit
}
impl From<bool> for Msg {
    fn from(value: bool) -> Self {
        if value { Msg::Draw }
        else { Msg::None }
    }
}

fn main() -> io::Result<()> {
    let mut term: Term = Terminal::classic(CrosstermBackend::default())?;
    let mut app = App::new();

    draw_ui(&mut term, &app)?;

    loop {
        let event = term.read_events()?;
        let msg = app.handle_events(event);

        match msg {
            Msg::None => continue,
            Msg::Draw => draw_ui(&mut term, &app)?,
            Msg::Resize(w, h) => resize(&mut term, &app, w, h)?,
            Msg::Quit => break
        }
    }

    Ok(())
}

fn draw_ui(term: &mut Term, app: &App) -> io::Result<()> {
    term.clear();

    let rect = term.rect();
    app.draw(&mut term.buffer, rect);

    term.draw()?;
    Ok(())
}
fn resize(term: &mut Term, app: &App, width: u16, height: u16) -> io::Result<()> {
    term.resize(width, height)?;
    draw_ui(term, app)
}
