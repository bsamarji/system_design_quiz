//! System Design Quiz
//!
//! System design quiz is a CLI application that tests a user's system design
//! knowledge by providing an interactive MCQ style quiz.
//!
//! This library crate exposes the public API that powers the
//! system_design_quiz

pub mod app_state;
pub mod models;
pub mod quiz;
pub mod storage;
pub mod terminal;

#[cfg(test)]
pub mod test_helpers;
