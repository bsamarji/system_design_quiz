//! Models
//!
//! The responsibility of the `models` crate is to define the custom data types
//! for the system_design_quiz application.

use serde::{Deserialize, Serialize};

/// One of four options to choose from when answering a question
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum Choice {
    A,
    B,
    C,
    D,
}

/// Data structure for holding data and metadata about a quiz question
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Question {
    /// Unique id for a question
    pub id: String,
    /// The quiz question
    pub prompt: String,
    /// The 4 answers the user can select
    pub options: [String; 4],
    /// The correct answer to the prompt
    pub correct_answer: Choice,
    /// Explanation for the correct answer
    pub explanation: String,
}
