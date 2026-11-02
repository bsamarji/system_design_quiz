//! Quiz
//!
//! The responsibility of the `quiz` crate is to load the questions, evaluate
//! answers and calculate quiz scores.

use crate::models;

/// JSON question bank included as a string
const QUESTION_BANK: &str = include_str!("../data/questions.json");

/// Load the question bank from the included JSON question bank and deserialize
/// the JSON data into a vector of `Question` structs
pub fn load_questions() -> Result<Vec<models::Question>, serde_json::Error> {
    serde_json::from_str(QUESTION_BANK)
}

#[cfg(test)]
mod tests {
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
}
