//! App State
//!
//! The responsibility of the `app_state` crate is to manage the loading,
//! rendering and progress of the application.

use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::models::{Choice, Question, QuizProgress};

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
    /// The current screen to render
    pub screen: Screen,
    /// Determines if the app should quit
    pub should_quit: bool,
    /// Key is the question id, value is the selected answer by the user
    pub answers: HashMap<String, Choice>,
}

/// Retrieve the current question index from the current quiz progress
///
/// If the question id can no longer be found in the question bank then
/// index position zero is returned by default
fn get_current_question_index(questions: &Vec<Question>, progress: &QuizProgress) -> usize {
    questions
        .iter()
        .position(|question| question.id == progress.current_question_id)
        .unwrap_or(0)
}

impl App {
    /// Constructs the app state for a new user
    pub fn build(questions: Vec<Question>) -> Self {
        Self {
            questions: questions,
            current_question_index: 0,
            screen: Screen::Question,
            should_quit: false,
            answers: HashMap::new(),
        }
    }

    /// Constructs the app state from loaded quiz progress
    pub fn build_from_progress(questions: Vec<Question>, progress: QuizProgress) -> Self {
        let current_question_index = get_current_question_index(&questions, &progress);

        let mut app = Self {
            questions: questions,
            current_question_index: current_question_index,
            screen: Screen::Question,
            should_quit: false,
            answers: progress.answers,
        };

        app.update_screen_for_current_question();
        app
    }

    /// Construct the current `QuizProgress` from the app state
    pub fn progress(&self) -> QuizProgress {
        QuizProgress {
            current_question_id: self.get_current_question_id(),
            answers: self.answers.clone(),
        }
    }

    /// Sets the `should_quit` value of the `App` to true
    pub fn quit(&mut self) {
        self.should_quit = true
    }

    /// Retrieve the current questions unique `String` id
    fn get_current_question_id(&self) -> String {
        let id = &self.questions[self.current_question_index].id;
        id.to_string()
    }

    /// Increments the `current_question_index` by 1
    ///
    /// If the `current_question_index` reaches the length of the question
    /// bank then it won't increment anymore
    fn increment_question_index(&mut self) {
        let total_questions = self.questions.len() - 1;

        if self.current_question_index < total_questions {
            self.current_question_index += 1
        }
    }

    /// Decrements the `current_question_index` by 1
    ///
    /// If the `current_question_index` reaches 0 then it won't decrement
    /// anymore
    fn decrement_question_index(&mut self) {
        if self.current_question_index > 0 {
            self.current_question_index -= 1
        }
    }

    /// Get the current answer to the current question
    ///
    /// Because the function returns an option, we can discern if the user has
    /// selected an answer or not
    pub fn current_answer(&self) -> Option<Choice> {
        self.answers.get(&self.get_current_question_id()).copied()
    }

    /// Decides if the current question has been answered
    ///
    /// If not then insert the answer into the answers hashmap
    ///
    /// If so then the function just returns
    fn answer_current_question(&mut self, answer: Choice) {
        if self.current_answer().is_none() {
            self.answers.insert(self.get_current_question_id(), answer);
            self.screen = Screen::Feedback;
        }
    }

    /// Determines the current screen based on state of the current answer
    /// to the current question
    fn update_screen_for_current_question(&mut self) {
        self.screen = if self.current_answer().is_some() {
            Screen::Feedback
        } else {
            Screen::Question
        };
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
                    self.answer_current_question(Choice::A);
                }
                KeyCode::Char('b') => {
                    self.answer_current_question(Choice::B);
                }
                KeyCode::Char('c') => {
                    self.answer_current_question(Choice::C);
                }
                KeyCode::Char('d') => {
                    self.answer_current_question(Choice::D);
                }
                KeyCode::Char('n') => {
                    self.increment_question_index();
                    self.update_screen_for_current_question();
                }
                KeyCode::Char('p') => {
                    self.decrement_question_index();
                    self.update_screen_for_current_question();
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{cap_theorem_question, scalability_question};

    #[test]
    fn get_current_question_index_from_quiz_progress() {
        let answers = HashMap::from([("scalability_001".to_string(), Choice::A)]);
        let progress = QuizProgress {
            current_question_id: "cap_theorem_001".to_string(),
            answers: answers,
        };
        let questions = vec![scalability_question(), cap_theorem_question()];

        let index = get_current_question_index(&questions, &progress);

        assert_eq!(index, 1);
    }

    #[test]
    fn build_app_state_from_new() {
        let test_app = App::build(vec![scalability_question()]);

        assert_eq!(test_app.questions[0], scalability_question());
        assert_eq!(test_app.screen, Screen::Question);
        assert_eq!(test_app.should_quit, false);
        assert_eq!(test_app.current_question_index, 0);
        assert!(test_app.answers.is_empty());
    }

    #[test]
    fn build_app_state_from_progress() {
        let answers = HashMap::from([("scalability_001".to_string(), Choice::A)]);
        let progress = QuizProgress {
            current_question_id: "cap_theorem_001".to_string(),
            answers: answers,
        };
        let questions = vec![scalability_question(), cap_theorem_question()];

        let test_app = App::build_from_progress(questions, progress);

        assert_eq!(
            test_app.questions,
            vec![scalability_question(), cap_theorem_question()]
        );
        assert_eq!(test_app.screen, Screen::Question);
        assert_eq!(test_app.should_quit, false);
        assert_eq!(test_app.current_question_index, 1);
        assert!(!test_app.answers.is_empty());
        assert_eq!(test_app.answers["scalability_001"], Choice::A);
    }

    #[test]
    fn get_progress_from_app_state() {
        let answers = HashMap::from([("scalability_001".to_string(), Choice::A)]);
        let progress = QuizProgress {
            current_question_id: "cap_theorem_001".to_string(),
            answers: answers,
        };
        let questions = vec![scalability_question(), cap_theorem_question()];

        let test_app = App::build_from_progress(questions, progress);

        let progress = test_app.progress();

        assert_eq!(progress.current_question_id, "cap_theorem_001".to_string());
        assert_eq!(progress.answers["scalability_001"], Choice::A);
    }

    #[test]
    fn set_quit_state() {
        let mut test_app = App::build(vec![scalability_question()]);
        test_app.quit();

        assert_eq!(test_app.should_quit, true);
    }

    #[test]
    fn get_current_question_id_from_app_state() {
        let test_app = App::build(vec![scalability_question()]);

        assert_eq!(
            test_app.get_current_question_id(),
            "scalability_001".to_string()
        );
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

        for (key, expected_choice) in cases {
            let mut test_app = App::build(vec![scalability_question()]);
            test_app.handle_event(KeyEvent::new(KeyCode::Char(key), KeyModifiers::NONE));

            assert_eq!(test_app.current_answer(), Some(expected_choice));
            assert_eq!(test_app.screen, Screen::Feedback);
            assert_eq!(test_app.answers["scalability_001"], expected_choice);
        }
    }

    #[test]
    fn answer_remains_first_key_press() {
        let cases = [('a', Choice::A), ('b', Choice::B)];
        let expected_choice = Choice::A;

        let mut test_app = App::build(vec![scalability_question()]);

        for (key, _) in cases {
            test_app.handle_event(KeyEvent::new(KeyCode::Char(key), KeyModifiers::NONE));

            assert_eq!(test_app.current_answer(), Some(expected_choice));
            assert_eq!(test_app.screen, Screen::Feedback);
            assert_eq!(test_app.answers["scalability_001"], expected_choice);
        }
    }

    #[test]
    fn load_progress_whose_current_question_has_answer() {
        let answers = HashMap::from([("scalability_001".to_string(), Choice::A)]);
        let progress = QuizProgress {
            current_question_id: "scalability_001".to_string(),
            answers: answers,
        };
        let questions = vec![scalability_question(), cap_theorem_question()];

        let test_app = App::build_from_progress(questions, progress);

        assert_eq!(test_app.screen, Screen::Feedback);
    }

    #[test]
    fn screen_is_derived_from_current_answer() {
        let mut test_app = App::build(vec![scalability_question(), cap_theorem_question()]);
        test_app.handle_event(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
        test_app.handle_event(KeyEvent::new(KeyCode::Char('n'), KeyModifiers::NONE));

        assert_eq!(
            test_app.screen,
            Screen::Question,
            "Expecting unanswered question to have Question screen"
        );

        test_app.handle_event(KeyEvent::new(KeyCode::Char('p'), KeyModifiers::NONE));

        assert_eq!(
            test_app.screen,
            Screen::Feedback,
            "Expecting answered question to have Feedback screen"
        );
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
    }

    #[test]
    fn p_moves_to_previous_question() {
        let mut test_app = App::build(vec![scalability_question(), cap_theorem_question()]);
        test_app.current_question_index = 1;

        test_app.handle_event(KeyEvent::new(KeyCode::Char('p'), KeyModifiers::NONE));

        assert_eq!(test_app.current_question_index, 0);
    }

    #[test]
    fn other_key_does_not_change_app_state() {
        let mut test_app = App::build(vec![scalability_question(), cap_theorem_question()]);

        test_app.handle_event(KeyEvent::new(KeyCode::Char('z'), KeyModifiers::NONE));

        assert_eq!(test_app.current_question_index, 0);
        assert_eq!(test_app.should_quit, false);
        assert_eq!(test_app.screen, Screen::Question);
        assert!(test_app.answers.is_empty());
    }
}
