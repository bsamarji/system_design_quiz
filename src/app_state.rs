//! App State
//!
//! The responsibility of the `app_state` crate is to manage the loading,
//! rendering and progress of the application.

use crate::models::{Choice, Question};

/// Determines what to display and what actions are available
#[derive(Debug, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::scalability_question;

    #[test]
    fn build_app_state() {
        let test_app = App::build(vec![scalability_question()]);

        assert_eq!(test_app.questions[0], scalability_question());
        assert_eq!(test_app.screen, Screen::Question);
        assert_eq!(test_app.selected_answer, None);
        assert_eq!(test_app.should_quit, false);
        assert_eq!(test_app.current_question_index, 0);
    }
}
