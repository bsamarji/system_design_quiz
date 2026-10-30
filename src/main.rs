use std::{io, process};
use system_design_quiz::app_state::App;
use system_design_quiz::quiz::load_questions;
use system_design_quiz::terminal::TerminalGuard;
use system_design_quiz::terminal::render;

fn main() {
    // Load question bank
    let questions = load_questions().unwrap_or_else(|err| {
        eprintln!("Error loading questions: {err}");
        process::exit(1);
    });

    // Initialize app state
    let mut app = App::build(questions);
    let mut stdout = io::stdout();

    // Enter app screen
    let _terminal = TerminalGuard::enter(&mut stdout);

    // Render current question
    render(&app.questions[app.current_question_index], &mut stdout).unwrap_or_else(|err| {
        eprintln!("Error rendering app: {err}");
        process::exit(1);
    })
}
