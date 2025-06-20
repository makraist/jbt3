use thiserror::Error;

#[derive(Error, Debug)]
pub enum SurveyError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Excel parsing error: {0}")]
    ExcelError(#[from] calamine::Error),

    #[error("Question not found with ID: {0}")]
    QuestionNotFound(usize),

    #[error("Invalid question type for operation")]
    InvalidQuestionType,

    #[error("Option not found: {0}")]
    OptionNotFound(String),

    #[error("Data parsing error: {0}")]
    DataParsingError(String),

    #[error("Empty dataset")]
    EmptyDataset,
}

pub type Result<T> = std::result::Result<T, SurveyError>;