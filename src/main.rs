use crossterm::event::{self, Event, KeyEventKind};
use std::{io, process};
use system_design_quiz::app_state::App;
use system_design_quiz::quiz::load_questions;
use system_design_quiz::terminal::{TerminalGuard, render};

fn main() {
    // Load question bank
    let questions = load_questions().unwrap_or_else(|err| {
        eprintln!("Error loading questions: {err}");
        process::exit(1);
    });

    // Initialize app state
    let mut app = App::build(questions);
    let mut stdout = io::stdout();

    // Enter app screen and activate terminal guard
    let _terminal = TerminalGuard::enter(&mut stdout);

    while !app.should_quit {
        // Render the current screen depending on the app status
        render(&mut app, &mut stdout).unwrap_or_else(|err| {
            eprintln!("Error rendering app: {err}");
            process::exit(1);
        });

        // Block process until key press event
        if let Event::Key(key_event) = event::read().unwrap_or_else(|err| {
            eprintln!("Error listening for key event: {err}");
            process::exit(1);
        }) {
            if key_event.kind == KeyEventKind::Press {
                // Handle key press event
                app.handle_event(key_event);
            }
        }
    }
}
