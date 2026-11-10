use crossterm::event::{self, Event, KeyEventKind};
use std::{fs, io, process};
use system_design_quiz::app_state::App;
use system_design_quiz::quiz::load_questions;
use system_design_quiz::storage::{get_progress_file_path, load, save};
use system_design_quiz::terminal::{TerminalGuard, render};

fn main() {
    // Load question bank
    let questions = load_questions().unwrap_or_else(|err| {
        eprintln!("Error loading questions: {err}");
        process::exit(1);
    });

    // Get the progress.json file path based on the user's OS
    let progress_file_path = get_progress_file_path().unwrap_or_else(|| {
        eprintln!("Unable to determine the application's progress file path");
        process::exit(1);
    });

    // Check if the data dir exists
    let data_dir = progress_file_path.parent().unwrap_or_else(|| {
        eprintln!("Unable to determine the application's data dir from the progress file path");
        process::exit(1);
    });
    // Create the data dir if it does not exist
    if !data_dir.exists() {
        fs::create_dir(data_dir).unwrap_or_else(|err| {
            eprintln!("Error creating data directory: {err}");
            process::exit(1);
        })
    }

    // Load the progress file
    let progress = load(&progress_file_path).unwrap_or_else(|err| {
        eprintln!("Error attempting to load progress file: {err}");
        process::exit(1);
    });

    // Initialize app state
    let mut app = match progress {
        // If progress file exists, then build app state from progress
        Some(p) => App::build_from_progress(questions, p),
        // Otherwise, build app state from new
        None => App::build(questions),
    };

    // Initialize stdout handle
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

        let _ = save(&progress_file_path, &app.progress())
            .unwrap_or_else(|err| eprintln!("Error saving current progress: {err}"));
    }
}
