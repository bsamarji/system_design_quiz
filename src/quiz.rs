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
    use super::*;

    #[test]
    fn question_bank_not_empty() {
        let questions = match load_questions() {
            Err(e) => panic!("Failed to load questions: {}", e),
            Ok(q) => q,
        };

        assert!(!questions.is_empty())
    }

    #[test]
    fn verify_question_schema() {
        let questions = match load_questions() {
            Err(e) => panic!("Failed to load questions: {}", e),
            Ok(q) => q,
        };

        let first_question = models::Question {
            id: "scalability_001".to_string(),
            prompt: "A service is scalable if...".to_string(),
            options: [
                "It can handle increased workload by adding resources while maintaining acceptable performance".to_string(),
                "It responds quickly to requests when the workload is low".to_string(),
                "It continues operating when an individual server fails".to_string(),
                "It automatically restarts after a software crash".to_string(),
            ],
            correct_answer: models::Choice::A,
            explanation: "Scalability is a service's ability to handle growing workload by adding resources, such as CPU, memory, or additional servers, while maintaining acceptable performance. For example, adding servers might allow it to support more concurrent users without unacceptable increases in response time. The improvement does not have to be exactly proportional to the resources added; that stronger property is called linear scalability. B describes low latency under light load, C describes fault tolerance, and D describes recovery.".to_string(),
        };

        assert_eq!(questions[0], first_question);
    }
}
