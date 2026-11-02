use std::thread::sleep;
use std::{io, process, time};
use system_design_quiz::app_state::App;
use system_design_quiz::quiz::evaluate_answer;
use system_design_quiz::quiz::load_questions;
use system_design_quiz::terminal::{TerminalGuard, display_feedback, render_question};

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
    render_question(&mut app, &mut stdout).unwrap_or_else(|err| {
        eprintln!("Error rendering app: {err}");
        process::exit(1);
    });

    let is_correct = evaluate_answer(&mut app);
    display_feedback(is_correct, &mut app, stdout).unwrap_or_else(|err| {
        eprintln!("Error displaying feedback: {err}");
        process::exit(1);
    });

    sleep(time::Duration::from_secs(5));
}
