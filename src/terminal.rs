//! Terminal
//!
//! The responsibility of the `terminal` crate is to display questions/results
//! and to capture user input.

use crate::models;

/// Format and display a quiz question
pub fn display_question(question: models::Question) {
    println!("Question: {}\n", question.id);
    println!("{}\n", question.prompt);
    println!("- A: {}", question.options[0]);
    println!("- B: {}", question.options[1]);
    println!("- C: {}", question.options[2]);
    println!("- D: {}", question.options[3]);
}
