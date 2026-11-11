//! Quiz
//!
//! The responsibility of the `quiz` crate is to load the questions, evaluate
//! answers and calculate quiz scores.

use std::collections::HashMap;

use crate::{
    app_state,
    models::{self, Choice, Question},
};

/// JSON question bank included as a string
const QUESTION_BANK: &str = include_str!("../data/questions.json");

/// Load the question bank from the included JSON question bank and deserialize
/// the JSON data into a vector of `Question` structs
pub fn load_questions() -> Result<Vec<models::Question>, serde_json::Error> {
    serde_json::from_str(QUESTION_BANK)
}

/// Evaluates the selected answer against the current question's correct answer
pub fn evaluate_answer(app: &mut app_state::App) -> bool {
    let correct_answer = &app.questions[app.current_question_index].correct_answer;
    match &app.current_answer() {
        Some(a) => a == correct_answer,
        None => false,
    }
}

/// Count the number of answers that are correct in the answers hashmap
fn correct_answer_count(questions: &[Question], answers: &HashMap<String, Choice>) -> usize {
    questions
        .iter()
        .filter(|question| {
            answers
                .get(&question.id)
                .is_some_and(|answer| *answer == question.correct_answer)
        })
        .count()
}

/// Data structure to hold the final quiz score
pub struct QuizScore {
    /// The number of correct answers
    pub correct: usize,
    /// The total number of questions
    pub total: usize,
    /// The score as a percentage
    pub percentage: f64,
    /// The score turned into a grade based on the US grading system
    pub grade: char,
}

/// Calculate the final quiz score based on the questions and answers
pub fn calculate_score(questions: &[Question], answers: &HashMap<String, Choice>) -> QuizScore {
    let correct = correct_answer_count(questions, answers);
    let total = questions.len();
    let percentage = correct as f64 / total as f64 * 100.0;
    let grade = if percentage >= 90.0 {
        'A'
    } else if percentage >= 80.0 {
        'B'
    } else if percentage >= 70.0 {
        'C'
    } else if percentage >= 60.0 {
        'D'
    } else if percentage >= 50.0 {
        'E'
    } else {
        'F'
    };

    QuizScore {
        correct,
        total,
        percentage,
        grade,
    }
}

#[cfg(test)]
mod tests {
    use crate::app_state::App;
    use crate::models::Choice;
    use crate::test_helpers::{cap_theorem_question, scalability_question};

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
        app.answers.insert("scalability_001".to_string(), Choice::A);
        assert!(evaluate_answer(&mut app))
    }

    #[test]
    fn evaluate_incorrect_scalability_answer() {
        let mut app = App::build(vec![scalability_question()]);
        app.answers.insert("scalability_001".to_string(), Choice::B);
        assert!(!evaluate_answer(&mut app))
    }

    #[test]
    fn one_out_of_two_answers_correct() {
        let mut app = App::build(vec![scalability_question(), cap_theorem_question()]);
        app.answers.insert("scalability_001".to_string(), Choice::A); // correct answer is A
        app.answers.insert("cap_theorem_001".to_string(), Choice::A); // correct answer is C
        let correct_answer_count = correct_answer_count(&app.questions, &app.answers);

        assert_eq!(correct_answer_count, 1);
    }

    #[test]
    fn calculate_grade_a() {
        let mut app = App::build(vec![scalability_question(), cap_theorem_question()]);
        app.answers.insert("scalability_001".to_string(), Choice::A); // correct answer is A
        app.answers.insert("cap_theorem_001".to_string(), Choice::C); // correct answer is C
        let score = calculate_score(&app.questions, &app.answers);

        assert_eq!(score.correct, 2);
        assert_eq!(score.total, 2);
        assert_eq!(score.percentage, 100.0);
        assert_eq!(score.grade, 'A');
    }
}
