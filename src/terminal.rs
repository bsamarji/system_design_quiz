//! Terminal
//!
//! The responsibility of the `terminal` crate is to display questions/results
//! and to capture user input.

use std::io::{self, Write};

use crate::{
    app_state::{self, Screen},
    models::{self, Choice},
    quiz::evaluate_answer,
};
use crossterm::{
    cursor::MoveTo,
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

/// Renders a `Screen` in the terminal's alternate screen.
///
/// The `Screen` is auto-formatted using the display functions.
///
/// The `Screen` can be of Question, Feedback or Summary, depending on the app
/// state.
pub fn render<W: Write>(app: &mut app_state::App, mut stdout: W) -> io::Result<()> {
    match app.screen {
        Screen::Question => {
            display_question(&app.questions[app.current_question_index], &mut stdout)?;
        }
        Screen::Feedback => {
            let is_correct = evaluate_answer(app);
            display_feedback(is_correct, app, stdout)?;
        }
        Screen::Summary => {
            todo!()
        }
    }

    Ok(())
}

/// Prints a question to stdout.
///
/// The question is formatted into a user-friendly quiz style.
pub fn display_question<W: Write>(question: &models::Question, mut stdout: W) -> io::Result<()> {
    queue!(stdout, Clear(ClearType::All), MoveTo(0, 0),)?;

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

    queue!(
        stdout,
        Print(
            "Press a - d to answer / n to move to next question / p to move to previous question / q to quit\r\n"
        ),
        Print("\r\n\r\n"),
    )?;

    stdout.flush()
}

/// Prints feedback to stdout
///
/// A little message is displayed along with the correct answer and explanation
pub fn display_feedback<W: Write>(
    is_correct: bool,
    app: &mut app_state::App,
    mut stdout: W,
) -> io::Result<()> {
    let current_question = &app.questions[app.current_question_index];
    let correct_choice = match current_question.correct_answer {
        Choice::A => 'A',
        Choice::B => 'B',
        Choice::C => 'C',
        Choice::D => 'D',
    };

    if is_correct {
        queue!(
            stdout,
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Color::Green),
            Print(format!(
                "The correct answer is {correct_choice}. Well done!"
            )),
            SetAttribute(Attribute::Reset),
            Print("\r\n\r\n"),
        )?;
    } else {
        queue!(
            stdout,
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Color::Red),
            Print(format!(
                "The correct answer is {correct_choice}. Read the explanation to learn more..."
            )),
            SetAttribute(Attribute::Reset),
            Print("\r\n\r\n"),
        )?;
    }

    queue!(
        stdout,
        SetAttribute(Attribute::Bold),
        Print("Explanation:"),
        Print("\r\n\r\n"),
        SetAttribute(Attribute::Reset),
        Print(&current_question.explanation),
        Print("\r\n\r\n"),
    )?;

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
