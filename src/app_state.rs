//! App State
//!
//! The responsibility of the `app_state` crate is to manage the loading,
//! rendering and progress of the application.

use crate::models::{Choice, Question};

/// Determines what to display and what actions are available
pub enum Screen {
    Question,
    Feedback,
    Summary,
}

/// Data structure for holding the current state of the app
pub struct App {
    pub questions: Vec<Question>,
    pub current_question_index: usize,
    pub selected_answer: Option<Choice>,
    pub screen: Screen,
    pub should_quit: bool,
}

impl App {
    /// Constructs the app state for a new user
    pub fn build(questions: Vec<Question>) -> App {
        App {
            questions: questions,
            current_question_index: 0,
            selected_answer: None,
            screen: Screen::Question,
            should_quit: false,
        }
    }
}

pub fn run() {
    todo!()
}
