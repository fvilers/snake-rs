use crossterm::{
    cursor::{self, SetCursorStyle},
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute, queue,
    style::{self},
    terminal::{self, ClearType},
};
use game::Game;
use point::Point;
use std::{
    io::{Result, Write},
    ops::Add,
    thread,
    time::Duration,
};

mod direction;
mod game;
mod point;

const GAME_COLUMNS: u16 = 30;
const GAME_ROWS: u16 = 15;

/// # Errors
///
/// Will return `Err` for any terminal related errors
pub fn run<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    execute!(w, terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    let ui_shift = Point::new(1, 4);

    queue!(
        w,
        style::ResetColor,
        terminal::Clear(ClearType::All),
        cursor::Hide,
        terminal::SetTitle("Snake"),
        cursor::MoveTo(1, 1),
        style::Print(format!(
            "{:^width$}",
            "Snake (press 'q' to exit)",
            width = GAME_COLUMNS as usize + 2
        )),
        cursor::MoveToNextLine(2),
        style::Print(format!("┌{}┐", "─".repeat(GAME_COLUMNS as usize))),
    )?;

    for _ in 0..GAME_ROWS {
        queue!(
            w,
            cursor::MoveToNextLine(1),
            style::Print(format!("│{}│", " ".repeat(GAME_COLUMNS as usize))),
        )?;
    }

    queue!(
        w,
        cursor::MoveToNextLine(1),
        style::Print(format!("└{}┘", "─".repeat(GAME_COLUMNS as usize))),
    )?;
    w.flush()?;

    let mut game = Game::new(GAME_COLUMNS, GAME_ROWS);
    let mut previous_prints: Vec<Point> = Vec::new();

    while !game.is_over() {
        queue!(
            w,
            cursor::MoveTo(0, 2),
            style::Print(format!(
                "{:^width$}",
                format!("Score: {}", pluralize(game.score(), "point", "points")),
                width = GAME_COLUMNS as usize + 2
            )),
        )?;

        let snake: Vec<Point> = game.snake().map(|p| p.add(ui_shift)).collect();

        // Erase previous printed points and avoid flickering if we clear the terminal on each loop repetition
        for (x, y) in previous_prints
            .iter()
            .filter(|p| !snake.contains(p))
            .map(|p| p.coords())
        {
            queue!(w, cursor::MoveTo(x, y), style::Print(" "),)?;
        }
        previous_prints.clear();

        for (x, y) in snake.iter().map(|p| p.coords()) {
            queue!(w, cursor::MoveTo(x, y), style::Print("@"),)?;
            previous_prints.push(Point::new(x, y));
        }

        if let Some(food) = game.food() {
            let (x, y) = food.add(ui_shift).coords();

            queue!(w, cursor::MoveTo(x, y), style::Print("Ȭ"),)?;
            previous_prints.push(Point::new(x, y));
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
    terminal::disable_raw_mode()?;

    if game.is_over() {
        println!(
            "Game over! Final score: {}.",
            pluralize(game.score(), "point", "points")
        );
    }

    Ok(())
}

fn pluralize(q: u32, singular: &str, plural: &str) -> String {
    format!("{q} {}", if q < 2 { singular } else { plural })
}
