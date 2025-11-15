use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Answer {
    pub letter: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Quiz {
    pub question: String,
    pub answers: Vec<Answer>,
    pub correct_answer: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuizFile {
    pub questions: Vec<Quiz>,
}
