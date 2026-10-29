//! Terminal
//!
//! The responsibility of the `terminal` crate is to display questions/results
//! and to capture user input.

use std::io::{self, Write, stdout};

use crate::models;
use crossterm::{
    cursor::MoveTo,
    event::{
        self,
        Event::{self},
        KeyCode, KeyEventKind, KeyModifiers,
    },
    execute, queue,
    style::{Attribute, Color, Print, SetAttribute, SetForegroundColor},
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};

pub fn run(question: models::Question) -> io::Result<()> {
    enable_raw_mode()?;

    display_question(question)?;

    loop {
        if let Event::Key(key_event) = event::read()?
            && key_event.kind == KeyEventKind::Press
        {
            match key_event.code {
                KeyCode::Char('q') => {
                    break;
                }
                KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                    break;
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;

    Ok(())
}

/// Prints a question to stdout
/// The question is formatted into a user-friendly quiz style.
pub fn display_question(question: models::Question) -> io::Result<()> {
    let mut stdout = io::stdout().lock();

    queue!(
        stdout,
        EnterAlternateScreen,
        Clear(ClearType::All),
        MoveTo(0, 0),
    )?;

    queue!(
        stdout,
        SetAttribute(Attribute::Bold),
        Print(format!("Question id: {}", question.id)),
        Print("\r\n\r\n"),
        Print(&question.prompt),
        SetAttribute(Attribute::Reset),
        Print("\r\n\r\n"),
    )?;

    for (label, option) in ['A', 'B', 'C', 'D'].iter().zip(question.options.iter()) {
        queue!(
            stdout,
            SetForegroundColor(Color::Cyan),
            Print(format!("  {label}. ")),
            SetAttribute(Attribute::Reset),
            Print(option),
            Print("\r\n\r\n"),
        )?;
    }

    queue!(stdout, Print("Press a-d to answer | q to quit\r\n"))?;

    stdout.flush()
}
