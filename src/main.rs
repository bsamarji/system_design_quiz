use std::process;
use system_design_quiz::quiz::load_questions;
use system_design_quiz::terminal::display_question;

fn main() {
    let questions = load_questions().unwrap_or_else(|err| {
        eprintln!("Error loading questions: {err}");
        process::exit(1);
    });

    for question in questions {
        display_question(question);
    }
}
