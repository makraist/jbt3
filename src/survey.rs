use std::collections::HashMap;
use std::path::Path;
use calamine::{Reader, Xlsx, open_workbook};
use serde::{Deserialize, Serialize};
use crate::error::{SurveyError, Result};
use crate::analysis::{AnswerDistribution, Subset};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestionType {
    SingleChoice,
    MultipleChoice,
    Text,
    Numeric,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: usize,
    pub text: String,
    pub question_type: QuestionType,
    pub options: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Answer {
    pub respondent_id: usize,
    pub question_id: usize,
    pub value: String,
}

#[derive(Debug)]
pub struct Survey {
    questions: Vec<Question>,
    answers: Vec<Answer>,
    respondent_count: usize,
}

impl Survey {
    /// Load survey data from an Excel file
    pub fn from_excel<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut workbook: Xlsx<_> = open_workbook(path)
            .map_err(calamine::Error::Xlsx)?;
        
        // Get the first worksheet (assuming survey data is in the first sheet)
        let worksheet_name = workbook.sheet_names().first()
            .ok_or_else(|| SurveyError::DataParsingError("No worksheets found".to_string()))?
            .clone();
        
        let range = workbook.worksheet_range(&worksheet_name)
            .map_err(calamine::Error::Xlsx)?;

        let mut questions = Vec::new();
        let mut answers = Vec::new();
        let mut headers = Vec::new();
        let mut respondent_count = 0;

        // Process the Excel data
        for (row_idx, row) in range.rows().enumerate() {
            if row_idx == 0 {
                // Header row - extract question information
                for (col_idx, cell) in row.iter().enumerate() {
                    let header = cell.to_string();
                    headers.push(header.clone());
                    
                    // Create question from header
                    let question = Question {
                        id: col_idx,
                        text: header.clone(),
                        question_type: Self::infer_question_type(&header),
                        options: Vec::new(), // Will be populated as we read answers
                    };
                    questions.push(question);
                }
            } else {
                // Data row - extract answers
                respondent_count = respondent_count.max(row_idx);
                
                for (col_idx, cell) in row.iter().enumerate() {
                    let value = cell.to_string().trim().to_string();
                    if !value.is_empty() && value != "NA" {
                        let answer = Answer {
                            respondent_id: row_idx - 1, // Subtract 1 because we skip header
                            question_id: col_idx,
                            value: value.clone(),
                        };
                        answers.push(answer);

                        // Update question options for multiple choice questions
                        if let Some(question) = questions.get_mut(col_idx) {
                            if matches!(question.question_type, QuestionType::SingleChoice | QuestionType::MultipleChoice) {
                                // Split on common separators for multiple choice
                                let options: Vec<String> = if value.contains(';') {
                                    value.split(';').map(|s| s.trim().to_string()).collect()
                                } else if value.contains(',') {
                                    value.split(',').map(|s| s.trim().to_string()).collect()
                                } else {
                                    vec![value]
                                };

                                for option in options {
                                    if !question.options.contains(&option) {
                                        question.options.push(option);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(Survey {
            questions,
            answers,
            respondent_count,
        })
    }

    /// Infer question type from header text
    fn infer_question_type(header: &str) -> QuestionType {
        let header_lower = header.to_lowercase();
        
        if header_lower.contains("select all") || header_lower.contains("multiple") {
            QuestionType::MultipleChoice
        } else if header_lower.contains("age") || header_lower.contains("years") || header_lower.contains("salary") {
            QuestionType::Numeric
        } else if header_lower.contains("describe") || header_lower.contains("other") || header_lower.contains("comment") {
            QuestionType::Text
        } else {
            QuestionType::SingleChoice
        }
    }

    /// Get all questions
    pub fn questions(&self) -> &[Question] {
        &self.questions
    }

    /// Get all answers
    pub fn answers(&self) -> &[Answer] {
        &self.answers
    }

    /// Get respondent count
    pub fn respondent_count(&self) -> usize {
        self.respondent_count
    }

    /// Search for questions containing a term
    pub fn search_questions(&self, term: &str) -> Vec<&Question> {
        let term_lower = term.to_lowercase();
        self.questions
            .iter()
            .filter(|q| q.text.to_lowercase().contains(&term_lower))
            .collect()
    }

    /// Search for answer options containing a term
    pub fn search_options(&self, term: &str) -> Vec<(usize, &str)> {
        let term_lower = term.to_lowercase();
        let mut results = Vec::new();
        
        for question in &self.questions {
            for option in &question.options {
                if option.to_lowercase().contains(&term_lower) {
                    results.push((question.id, option.as_str()));
                }
            }
        }
        
        results
    }

    /// Create a subset based on question and option
    pub fn create_subset(&self, question_id: usize, option: &str) -> Result<Subset> {
        let question = self.questions.get(question_id)
            .ok_or(SurveyError::QuestionNotFound(question_id))?;

        let matching_respondents: Vec<usize> = self.answers
            .iter()
            .filter(|a| a.question_id == question_id)
            .filter(|a| {
                // For multiple choice, check if option is contained in the answer
                if matches!(question.question_type, QuestionType::MultipleChoice) {
                    a.value.contains(option)
                } else {
                    a.value == option
                }
            })
            .map(|a| a.respondent_id)
            .collect();

        Ok(Subset {
            question_id,
            option: option.to_string(),
            respondent_ids: matching_respondents,
            total_respondents: self.respondent_count,
        })
    }

    /// Get answer distribution for a question
    pub fn get_distribution(&self, question_id: usize) -> Result<AnswerDistribution> {
        let question = self.questions.get(question_id)
            .ok_or(SurveyError::QuestionNotFound(question_id))?;

        let question_answers: Vec<&Answer> = self.answers
            .iter()
            .filter(|a| a.question_id == question_id)
            .collect();

        let mut counts = HashMap::new();
        let mut total_responses = 0;

        match question.question_type {
            QuestionType::SingleChoice => {
                for answer in question_answers {
                    *counts.entry(answer.value.clone()).or_insert(0) += 1;
                    total_responses += 1;
                }
            }
            QuestionType::MultipleChoice => {
                for answer in question_answers {
                    // Split multiple choice answers
                    let options: Vec<&str> = if answer.value.contains(';') {
                        answer.value.split(';').collect()
                    } else if answer.value.contains(',') {
                        answer.value.split(',').collect()
                    } else {
                        vec![answer.value.as_str()]
                    };

                    for option in options {
                        let option = option.trim();
                        if !option.is_empty() {
                            *counts.entry(option.to_string()).or_insert(0) += 1;
                        }
                    }
                    total_responses += 1;
                }
            }
            _ => {
                return Err(SurveyError::InvalidQuestionType);
            }
        }

        let mut distribution = HashMap::new();
        for (option, count) in counts {
            let percentage = if total_responses > 0 {
                (count as f64 / total_responses as f64) * 100.0
            } else {
                0.0
            };
            distribution.insert(option, (count, percentage));
        }

        Ok(AnswerDistribution {
            question_id,
            question_text: question.text.clone(),
            question_type: question.question_type.clone(),
            distribution,
            total_responses,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_question_type_inference() {
        assert!(matches!(Survey::infer_question_type("Select all that apply"), QuestionType::MultipleChoice));
        assert!(matches!(Survey::infer_question_type("What is your age?"), QuestionType::Numeric));
        assert!(matches!(Survey::infer_question_type("Please describe"), QuestionType::Text));
        assert!(matches!(Survey::infer_question_type("What is your role?"), QuestionType::SingleChoice));
    }

    #[test]
    fn test_search_functionality() {
        let questions = vec![
            Question {
                id: 0,
                text: "What programming language do you use?".to_string(),
                question_type: QuestionType::SingleChoice,
                options: vec!["Rust".to_string(), "Python".to_string()],
            },
            Question {
                id: 1,
                text: "How many years of experience?".to_string(),
                question_type: QuestionType::Numeric,
                options: vec![],
            },
        ];

        let survey = Survey {
            questions,
            answers: vec![],
            respondent_count: 0,
        };

        let results = survey.search_questions("programming");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, 0);

        let option_results = survey.search_options("rust");
        assert_eq!(option_results.len(), 1);
        assert_eq!(option_results[0], (0, "Rust"));
    }
}