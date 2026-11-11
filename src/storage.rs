//! Storage
//!
//! The responsibility of the `storage` crate is to safely read and write user
//! data to disk.

use directories::ProjectDirs;
use std::fs::{self};
use std::io::{self, Error, ErrorKind, Write};
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;

use crate::models::QuizProgress;

/// Saves the user's current quiz progress to a JSON file
///
/// The function will overwrite an existing progress file
pub fn save<P: AsRef<Path>>(file_path: P, progress: &QuizProgress) -> io::Result<()> {
    let file_path = file_path.as_ref();

    let parent_dir = file_path
        .parent()
        .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "File path has no parent directory"))?;

    let mut temp_file = NamedTempFile::new_in(parent_dir)?;

    serde_json::to_writer_pretty(&mut temp_file, progress)?;
    temp_file.flush()?;

    temp_file.persist(file_path)?;

    Ok(())
}

/// Load the user's current quiz progress from a JSON file
///
/// If the progress file does not exist, then the function will return `None`
pub fn load(file_path: &Path) -> io::Result<Option<QuizProgress>> {
    match fs::read_to_string(file_path) {
        Ok(file_contents) => {
            let progress: QuizProgress = serde_json::from_str(&file_contents)?;
            Ok(Some(progress))
        }
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error),
    }
}

/// Retrieve the path to the application's local progress file
pub fn get_progress_file_path() -> Option<PathBuf> {
    let project_dir = ProjectDirs::from("", "", "sdquiz")?;

    Some(project_dir.data_local_dir().join("progress.json"))
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;
    use crate::models::Choice;
    use std::{collections::HashMap, ffi::OsStr};

    #[test]
    fn save_quiz_progress() {
        let file = NamedTempFile::new().expect("Expected to create a temp file");
        let file_path = file.into_temp_path();
        let progress = QuizProgress {
            current_question_id: "cap_theorem_001".to_string(),
            answers: HashMap::from([("scalability_001".to_string(), Choice::A)]),
        };

        let result = save(&file_path, &progress);
        assert!(result.is_ok(), "Failed to save quiz progress");

        let file_contents = fs::read_to_string(file_path).expect("Expected to read temp file");
        assert!(file_contents.contains("current_question_id"));
        assert!(file_contents.contains("cap_theorem_001"));
        assert!(file_contents.contains("answers"));
        assert!(file_contents.contains("scalability_001"));
        assert!(file_contents.contains("0"));
    }

    #[test]
    fn load_quiz_progress() {
        let mut file = NamedTempFile::new().expect("Expected to create a temp file");
        let progress = QuizProgress {
            current_question_id: "cap_theorem_001".to_string(),
            answers: HashMap::from([("scalability_001".to_string(), Choice::A)]),
        };

        serde_json::to_writer_pretty(&file, &progress)
            .expect("Expected to write quiz progress to the file buffer");
        file.flush()
            .expect("Expected to flush the buffer and write to temp file");

        let result = load(file.path())
            .expect("Expected to load the quiz progress from the temp file")
            .unwrap();

        assert_eq!(result.current_question_id, "cap_theorem_001".to_string());
        assert_eq!(result.answers["scalability_001"], Choice::A);
    }

    #[test]
    fn load_quiz_progress_from_non_existent_file() {
        let dir = tempdir().expect("Expected to create a temp dir");
        let missing_file_path = dir.path().join("progress.json");

        let result = load(&missing_file_path).expect("Expected no error on load");

        assert!(result.is_none());
    }

    #[test]
    fn get_data_dir_path_and_json_file_name() {
        let progress_file_path = get_progress_file_path()
            .expect("Expected major operating systems to provide a project data directory");

        assert!(progress_file_path.is_absolute());

        assert_eq!(
            progress_file_path.file_name(),
            Some(OsStr::new("progress.json"))
        );

        assert!(
            progress_file_path
                .ancestors()
                .any(|path| path.file_name() == Some(OsStr::new("sdquiz"))),
            "Progress file should be stored beneath the sdquiz application directory"
        );
    }
}
