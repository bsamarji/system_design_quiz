//! Terminal
//!
//! The responsibility of the `terminal` crate is to display questions/results
//! and to capture user input.

use std::io::{self, Write};

use crate::{app_state, models};
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
    pub fn enter<W: Write>(mut out: W) -> io::Result<Self> {
        enable_raw_mode()?;

        if let Err(err) = execute!(out, EnterAlternateScreen) {
            let _ = disable_raw_mode()?;
            return Err(err);
        }

        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    /// Destructor for the `TerminalGuard`.
    ///
    /// Implements the drop trait for `TerminalGuard`.
    ///
    /// This destructor disables raw mode and leaves the alternate screen.
    fn drop(&mut self) {
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        let _ = disable_raw_mode();
    }
}

/// Renders a question in the terminal's alternate screen.
/// The question is auto-formatted using the `display_question` function
pub fn render<W: Write>(app: &mut app_state::App, mut stdout: W) -> io::Result<()> {
    queue!(stdout, Clear(ClearType::All), MoveTo(0, 0),)?;
    stdout.flush()?;

    display_question(&app.questions[app.current_question_index], &mut stdout)?;

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
                KeyCode::Char('a') => {
                    app.selected_answer = Some(models::Choice::A);
                    break;
                }
                KeyCode::Char('b') => {
                    app.selected_answer = Some(models::Choice::B);
                    break;
                }
                KeyCode::Char('c') => {
                    app.selected_answer = Some(models::Choice::C);
                    break;
                }
                KeyCode::Char('d') => {
                    app.selected_answer = Some(models::Choice::D);
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
pub fn display_question<W: Write>(question: &models::Question, mut stdout: W) -> io::Result<()> {
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

#[cfg(test)]
mod tests {
    use crossterm::terminal::is_raw_mode_enabled;

    use super::*;

    #[test]
    fn terminal_guard_drops_raw_mode() {
        {
            let mut buffer = Vec::new();
            let terminal = TerminalGuard::enter(&mut buffer);
            assert!(terminal.is_ok());

            let is_enabled = is_raw_mode_enabled().expect("Error retrieving raw mode status");
            assert!(is_enabled, "Terminal guard entry should enable raw mode");

            let mut expected = Vec::new();
            execute!(expected, EnterAlternateScreen).unwrap();

            assert_eq!(
                buffer, expected,
                "Terminal guard entry should enter alternate screen"
            );
        }
        // Drop currently writes to real stdout, so testing for LeaveAlternateScreen
        // is not viable until TerminalGuard is refactored
        let is_enabled = is_raw_mode_enabled().expect("Error retrieving raw mode status");
        assert!(!is_enabled, "Terminal guard drop should disable raw mode");
    }
}
