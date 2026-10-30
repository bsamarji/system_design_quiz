//! Terminal
//!
//! The responsibility of the `terminal` crate is to display questions/results
//! and to capture user input.

use std::io::{self, Write};

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

/// This struct can be used to enter and leave the alternate screen.
///
/// The struct only has a constructor and destructor. The destructor
/// implements the `Drop` trait and is automatically called when the
/// struct goes out of scope.
pub struct TerminalGuard;

impl TerminalGuard {
    /// Constructor for the `TerminalGuard`.
    ///
    /// Enables raw mode and switches to the alternate screen.
    pub fn enter() -> io::Result<Self> {
        enable_raw_mode()?;

        if let Err(err) = execute!(io::stdout(), EnterAlternateScreen) {
            let _ = disable_raw_mode()?;
            return Err(err);
        }

        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    /// Destructor for the `TerminalGuard`.
    ///
    /// Implements the drop trait and disables raw mode and leaves the alternate
    /// screen.
    fn drop(&mut self) {
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        let _ = disable_raw_mode();
    }
}

/// Renders a question in the terminal's alternate screen.
/// The question is auto-formatted using the `display_question` function
pub fn render(question: &models::Question) -> io::Result<()> {
    let mut stdout = io::stdout();

    queue!(stdout, Clear(ClearType::All), MoveTo(0, 0),)?;
    stdout.flush()?;

    display_question(question, &stdout)?;

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

    Ok(())
}

/// Prints a question to stdout
/// The question is formatted into a user-friendly quiz style.
pub fn display_question(question: &models::Question, mut stdout: &io::Stdout) -> io::Result<()> {
    queue!(
        stdout,
        SetAttribute(Attribute::Bold),
        SetAttribute(Attribute::Underlined),
        Print(format!("Question ID: {}", question.id)),
        SetAttribute(Attribute::Reset),
        Print("\r\n\r\n"),
    )?;

    queue!(
        stdout,
        SetAttribute(Attribute::Bold),
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

    queue!(stdout, Print("Press a-d to answer / q to quit\r\n"))?;

    stdout.flush()
}
