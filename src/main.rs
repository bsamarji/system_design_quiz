use std::process;
use system_design_quiz::app_state::App;
use system_design_quiz::quiz::load_questions;
use system_design_quiz::terminal::run;

fn main() {
    let mut app = App::new();
    let mut questions = load_questions().unwrap_or_else(|err| {
        eprintln!("Error loading questions: {err}");
        process::exit(1);
    });

    app.questions.append(&mut questions);

    for question in app.questions {
        run(question).unwrap_or_else(|err| {
            eprintln!("Error rendering app: {err}");
            process::exit(1);
        })
    }
}
