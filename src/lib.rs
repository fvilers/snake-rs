use crossterm::{
    cursor::{self, SetCursorStyle},
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute, queue, style,
    terminal::{self, ClearType},
};
use game::Game;
use point::Point;
use std::{
    io::{Result, Write},
    thread,
    time::Duration,
};

mod direction;
mod game;
mod point;

/// # Errors
///
/// Will return `Err` for any terminal related errors
pub fn run<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    execute!(w, terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    let (columns, rows) = terminal::size()?;
    let mut game = Game::new(columns, rows);

    loop {
        queue!(
            w,
            style::ResetColor,
            terminal::Clear(ClearType::All),
            cursor::Hide,
        )?;

        for (x, y) in game.snake().map(Point::coords) {
            queue!(w, cursor::MoveTo(x, y), style::Print("O"),)?;
        }

        w.flush()?;

        if event::poll(Duration::from_secs(0))? {
            if let Event::Key(KeyEvent {
                code,
                modifiers: _,
                kind: KeyEventKind::Press,
                state: _,
            }) = event::read()?
            {
                match code {
                    KeyCode::Up => game.up(),
                    KeyCode::Right => game.right(),
                    KeyCode::Down => game.down(),
                    KeyCode::Left => game.left(),
                    KeyCode::Char('q') => {
                        execute!(w, SetCursorStyle::DefaultUserShape)?;
                        break;
                    }
                    _ => {}
                }
            }
        }

        game.tick();
        thread::sleep(Duration::from_millis(250));
    }

    execute!(
        w,
        style::ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen
    )?;
    terminal::disable_raw_mode()
}
