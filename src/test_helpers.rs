//! Test Helpers
//!
//! The responsibility of the `test_helpers` crate  is to expose test helper
//! functions.
//! They allow tests to focus on function behaviour instead of heavy setup.

use crate::models;

/// Models the scalability_001 question from the question bank.
///
/// This is an exact 1 for 1 of the scalability_001 in the JSON question bank.
pub fn scalability_question() -> models::Question {
    models::Question {
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
    }
}
