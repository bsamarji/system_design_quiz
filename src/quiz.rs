//! Quiz
//!
//! The responsibility of the `quiz` crate is to load the questions, evaluate
//! answers and calculate quiz scores.

use crate::{app_state, models};

/// JSON question bank included as a string
const QUESTION_BANK: &str = include_str!("../data/questions.json");

/// Load the question bank from the included JSON question bank and deserialize
/// the JSON data into a vector of `Question` structs
pub fn load_questions() -> Result<Vec<models::Question>, serde_json::Error> {
    serde_json::from_str(QUESTION_BANK)
}

pub fn evaluate_answer(app: &mut app_state::App) -> bool {
    let correct_answer = &app.questions[app.current_question_index].correct_answer;
    match &app.selected_answer {
        Some(s) => s == correct_answer,
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use crate::app_state::App;
    use crate::models::Choice;
    use crate::test_helpers::scalability_question;

    use super::*;

    #[test]
    fn loads_a_non_empty_question_bank() {
        let questions = load_questions().expect("The bundled question bank should be valid JSON");

        assert!(!questions.is_empty())
    }

    #[test]
    fn loads_the_scalability_question() {
        let questions = load_questions().expect("The bundled question bank should be valid JSON");

        assert!(questions.contains(&scalability_question()));
    }

    #[test]
    fn evaluate_correct_scalability_answer() {
        let mut app = App::build(vec![scalability_question()]);
        app.selected_answer = Some(Choice::A);
        assert!(evaluate_answer(&mut app))
    }

    #[test]
    fn evaluate_incorrect_scalability_answer() {
        let mut app = App::build(vec![scalability_question()]);
        app.selected_answer = Some(Choice::B);
        assert!(!evaluate_answer(&mut app))
    }
}
