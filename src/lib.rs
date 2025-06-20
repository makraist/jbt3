use std::path::Path;

pub mod error;
pub mod survey;
pub mod analysis;

pub use error::SurveyError;
pub use survey::{Survey, Question, QuestionType, Answer};
pub use analysis::{AnswerDistribution, Subset};

/// Main entry point for the Stack Overflow Survey Analyzer library
pub struct SurveyAnalyzer {
    survey: Survey,
}

impl SurveyAnalyzer {
    /// Create a new SurveyAnalyzer by loading data from an Excel file
    pub fn from_excel<P: AsRef<Path>>(path: P) -> Result<Self, SurveyError> {
        let survey = Survey::from_excel(path)?;
        Ok(Self { survey })
    }

    /// Get the survey structure (list of questions)
    pub fn get_survey_structure(&self) -> &[Question] {
        self.survey.questions()
    }

    /// Search for questions containing the given term
    pub fn search_questions(&self, term: &str) -> Vec<&Question> {
        self.survey.search_questions(term)
    }

    /// Search for answer options containing the given term
    pub fn search_options(&self, term: &str) -> Vec<(usize, &str)> {
        self.survey.search_options(term)
    }

    /// Create a subset of respondents based on question and answer option
    pub fn create_subset(&self, question_id: usize, option: &str) -> Result<Subset, SurveyError> {
        self.survey.create_subset(question_id, option)
    }

    /// Get answer distribution for a question
    pub fn get_distribution(&self, question_id: usize) -> Result<AnswerDistribution, SurveyError> {
        self.survey.get_distribution(question_id)
    }

    /// Get the raw survey data
    pub fn survey(&self) -> &Survey {
        &self.survey
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_analyzer_creation() {
        // This test will be implemented after we have the core functionality
        assert!(true);
    }
}
