use crate::models::{Answer, Quiz, QuizFile};
use std::fs;

pub fn load_quizzes(path: &str) -> Vec<Quiz> {
    let content = fs::read_to_string(path).unwrap_or_else(|_| {
        r#"questions:
  - question: "Question par défaut ?"
    answers:
      - letter: A
        text: "Réponse A"
      - letter: B
        text: "Réponse B"
      - letter: C
        text: "Réponse C"
      - letter: D
        text: "Réponse D"
    correct_answer: A
"#.to_string()
    });
    
    let quiz_file: QuizFile = serde_yaml::from_str(&content).unwrap_or_else(|_| {
        QuizFile {
            questions: vec![Quiz {
                question: "Question par défaut ?".to_string(),
                answers: vec![
                    Answer { letter: "A".to_string(), text: "Réponse A".to_string(), image: None },
                    Answer { letter: "B".to_string(), text: "Réponse B".to_string(), image: None },
                    Answer { letter: "C".to_string(), text: "Réponse C".to_string(), image: None },
                    Answer { letter: "D".to_string(), text: "Réponse D".to_string(), image: None },
                ],
                correct_answer: "A".to_string(),
            }],
        }
    });
    
    quiz_file.questions
}

pub fn load_quiz_list(dir: &str) -> Vec<String> {
    let mut quiz_files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_file() {
                    if let Some(name) = entry.file_name().to_str() {
                        if name.ends_with(".txt") || name.ends_with(".yaml") {
                            quiz_files.push(name.to_string());
                        }
                    }
                }
            }
        }
    }
    quiz_files.sort();
    quiz_files
}
