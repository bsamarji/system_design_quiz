//! App State
//!
//! The responsibility of the `app_state` crate is to manage the loading,
//! rendering and progress of the application.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

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
    /// Holds the `Question` bank in a vector
    pub questions: Vec<Question>,
    /// The index position pointing to the current question
    pub current_question_index: usize,
    /// Holds the user's selected answer
    pub selected_answer: Option<Choice>,
    /// The current screen to render
    pub screen: Screen,
    /// Determines if the app should quit
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

    /// Sets the `should_quit` value of the `App` to true
    pub fn quit(&mut self) {
        self.should_quit = true
    }

    /// Handles key press events to dictate the next action the application
    /// takes
    pub fn handle_event(&mut self, key_event: KeyEvent) {
        {
            match key_event.code {
                KeyCode::Char('q') => {
                    self.quit();
                }
                KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.quit();
                }
                KeyCode::Char('a') => {
                    self.selected_answer = Some(Choice::A);
                    self.screen = Screen::Feedback;
                }
                KeyCode::Char('b') => {
                    self.selected_answer = Some(Choice::B);
                    self.screen = Screen::Feedback;
                }
                KeyCode::Char('c') => {
                    self.selected_answer = Some(Choice::C);
                    self.screen = Screen::Feedback;
                }
                KeyCode::Char('d') => {
                    self.selected_answer = Some(Choice::D);
                    self.screen = Screen::Feedback;
                }
                KeyCode::Char('n') => {
                    self.increment_question_index();
                    self.screen = Screen::Question;
                }
                KeyCode::Char('p') => {
                    self.decrement_question_index();
                    self.screen = Screen::Question;
                }
                _ => {}
            }
        }
    }

    /// Increments the `current_question_index` by 1
    ///
    /// If the `current_question_index` reaches the length of the question
    /// bank then it won't increment anymore
    pub fn increment_question_index(&mut self) {
        let total_questions = self.questions.len() - 1;

        if self.current_question_index < total_questions {
            self.current_question_index += 1
        }
    }

    /// Decrements the `current_question_index` by 1
    ///
    /// If the `current_question_index` reaches 0 then it won't decrement
    /// anymore
    pub fn decrement_question_index(&mut self) {
        if self.current_question_index > 0 {
            self.current_question_index -= 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{cap_theorem_question, scalability_question};

    #[test]
    fn build_app_state() {
        let test_app = App::build(vec![scalability_question()]);

        assert_eq!(test_app.questions[0], scalability_question());
        assert_eq!(test_app.screen, Screen::Question);
        assert_eq!(test_app.selected_answer, None);
        assert_eq!(test_app.should_quit, false);
        assert_eq!(test_app.current_question_index, 0);
    }

    #[test]
    fn set_quit_state() {
        let mut test_app = App::build(vec![scalability_question()]);
        test_app.quit();

        assert_eq!(test_app.should_quit, true);
    }

    #[test]
    fn increment_question_index_by_one() {
        let mut test_app = App::build(vec![scalability_question(), cap_theorem_question()]);
        // Increment the question index
        test_app.increment_question_index();

        assert_eq!(test_app.current_question_index, 1);

        // Call the increment method again, however since we have reached the
        // maximum length of the questions vector, the method should not
        // increase the index and it should remain the same.
        test_app.increment_question_index();

        assert_eq!(test_app.current_question_index, 1);
    }

    #[test]
    fn decrement_question_index_by_one() {
        let mut test_app = App::build(vec![scalability_question(), cap_theorem_question()]);
        test_app.current_question_index = 1;

        // Decrement the question index
        test_app.decrement_question_index();

        assert_eq!(test_app.current_question_index, 0);

        // Call the decrement method again, however since we have reached
        // position zero, the method should not decrement the index and
        // it should remain at zero.
        test_app.decrement_question_index();

        assert_eq!(test_app.current_question_index, 0);
    }

    #[test]
    fn answer_key_select_matching_choice() {
        let cases = [
            ('a', Choice::A),
            ('b', Choice::B),
            ('c', Choice::C),
            ('d', Choice::D),
        ];

        let mut test_app = App::build(vec![scalability_question()]);

        for (key, expected_choice) in cases {
            test_app.handle_event(KeyEvent::new(KeyCode::Char(key), KeyModifiers::NONE));

            assert_eq!(test_app.selected_answer, Some(expected_choice));
            assert_eq!(test_app.screen, Screen::Feedback);
        }
    }

    #[test]
    fn q_sets_exit_status_true() {
        let mut test_app = App::build(vec![scalability_question()]);

        test_app.handle_event(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE));

        assert_eq!(test_app.should_quit, true);
    }

    #[test]
    fn ctrl_c_sets_exit_status_true() {
        let mut test_app = App::build(vec![scalability_question()]);

        test_app.handle_event(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL));

        assert_eq!(test_app.should_quit, true);
    }

    #[test]
    fn n_moves_to_next_question() {
        let mut test_app = App::build(vec![scalability_question(), cap_theorem_question()]);

        test_app.handle_event(KeyEvent::new(KeyCode::Char('n'), KeyModifiers::NONE));

        assert_eq!(test_app.current_question_index, 1);
        assert_eq!(test_app.screen, Screen::Question);
    }

    #[test]
    fn p_moves_to_previous_question() {
        let mut test_app = App::build(vec![scalability_question(), cap_theorem_question()]);
        test_app.current_question_index = 1;

        test_app.handle_event(KeyEvent::new(KeyCode::Char('p'), KeyModifiers::NONE));

        assert_eq!(test_app.current_question_index, 0);
        assert_eq!(test_app.screen, Screen::Question);
    }

    #[test]
    fn other_key_does_not_change_app_state() {
        let mut test_app = App::build(vec![scalability_question(), cap_theorem_question()]);

        test_app.handle_event(KeyEvent::new(KeyCode::Char('z'), KeyModifiers::NONE));

        assert_eq!(test_app.current_question_index, 0);
        assert_eq!(test_app.selected_answer, None);
        assert_eq!(test_app.should_quit, false);
        assert_eq!(test_app.screen, Screen::Question);
    }
}
