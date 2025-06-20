use anyhow::{anyhow, Result};
use calamine::{open_workbook, Reader, Xlsx};
use indexmap::IndexMap;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuestionType {
    SC, // Single Choice
    MC, // Multiple Choice
    TE, // Text Entry
}

impl From<&str> for QuestionType {
    fn from(s: &str) -> Self {
        match s {
            "SC" => QuestionType::SC,
            "MC" => QuestionType::MC,
            "TE" => QuestionType::TE,
            _ => QuestionType::TE, // Default to text entry
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurveyQuestion {
    pub column: String,
    pub question_text: String,
    pub question_type: QuestionType,
}

#[derive(Debug, Clone)]
pub struct SurveyData {
    pub questions: IndexMap<String, SurveyQuestion>,
    pub responses: Vec<IndexMap<String, String>>,
}

impl SurveyData {
    /// Load survey data from an Excel file
    pub fn load_from_excel<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut workbook: Xlsx<_> = open_workbook(path)?;

        // Load schema (questions metadata)
        let mut questions = IndexMap::new();
        if let Ok(range) = workbook.worksheet_range("schema") {
            let mut rows = range.rows();

            // Skip header row
            if let Some(_header) = rows.next() {
                for row in rows {
                    if row.len() >= 3 {
                        let column = row[0].to_string();
                        let question_text = row[1].to_string();
                        let question_type = QuestionType::from(row[2].to_string().as_str());

                        questions.insert(
                            column.clone(),
                            SurveyQuestion {
                                column,
                                question_text,
                                question_type,
                            },
                        );
                    }
                }
            }
        }

        // Load raw data (responses)
        let mut responses = Vec::new();
        if let Ok(range) = workbook.worksheet_range("raw data") {
            let mut rows = range.rows();

            // Get header row to map column positions
            let headers: Vec<String> = if let Some(header_row) = rows.next() {
                header_row.iter().map(|cell| cell.to_string()).collect()
            } else {
                return Err(anyhow!("No header row found in raw data"));
            };

            // Process each response row
            for row in rows {
                let mut response = IndexMap::new();
                for (i, cell) in row.iter().enumerate() {
                    if i < headers.len() {
                        response.insert(headers[i].clone(), cell.to_string());
                    }
                }
                responses.push(response);
            }
        }

        Ok(SurveyData {
            questions,
            responses,
        })
    }

    /// Display the survey structure (list of questions)
    pub fn display_survey_structure(&self) {
        println!("=== Stack Overflow Survey Structure ===");
        println!("Total questions: {}", self.questions.len());
        println!("Total responses: {}", self.responses.len());
        println!();

        for (i, (column, question)) in self.questions.iter().enumerate() {
            println!(
                "{}. [{}] {} (Type: {:?})",
                i + 1,
                column,
                question.question_text,
                question.question_type
            );
        }
    }

    /// Search for questions by keyword in question text or column name
    pub fn search_questions(&self, keyword: &str) -> Vec<&SurveyQuestion> {
        let keyword_lower = keyword.to_lowercase();
        let regex = Regex::new(&keyword_lower)
            .unwrap_or_else(|_| Regex::new(&regex::escape(&keyword_lower)).unwrap());

        self.questions
            .values()
            .filter(|question| {
                regex.is_match(&question.column.to_lowercase())
                    || regex.is_match(&question.question_text.to_lowercase())
            })
            .collect()
    }

    /// Create subset of respondents based on question and option
    pub fn create_subset(
        &self,
        column: &str,
        option: &str,
    ) -> Result<Vec<&IndexMap<String, String>>> {
        if !self.questions.contains_key(column) {
            return Err(anyhow!("Column '{}' not found", column));
        }

        let question = &self.questions[column];
        let subset: Vec<&IndexMap<String, String>> = match question.question_type {
            QuestionType::SC | QuestionType::TE => {
                // For single choice and text entry, exact match
                self.responses
                    .iter()
                    .filter(|response| {
                        response
                            .get(column)
                            .map(|value| value == option)
                            .unwrap_or(false)
                    })
                    .collect()
            }
            QuestionType::MC => {
                // For multiple choice, check if option is contained in the response
                self.responses
                    .iter()
                    .filter(|response| {
                        response
                            .get(column)
                            .map(|value| value.contains(option))
                            .unwrap_or(false)
                    })
                    .collect()
            }
        };

        Ok(subset)
    }

    /// Display distribution of answers for a question
    pub fn display_distribution(&self, column: &str) -> Result<()> {
        if !self.questions.contains_key(column) {
            return Err(anyhow!("Column '{}' not found", column));
        }

        let question = &self.questions[column];
        println!("=== Distribution for: {} ===", question.question_text);
        println!("Question Type: {:?}", question.question_type);
        println!();

        match question.question_type {
            QuestionType::SC | QuestionType::TE => {
                self.display_single_choice_distribution(column)?;
            }
            QuestionType::MC => {
                self.display_multiple_choice_distribution(column)?;
            }
        }

        Ok(())
    }

    fn display_single_choice_distribution(&self, column: &str) -> Result<()> {
        let mut counts: HashMap<String, usize> = HashMap::new();
        let mut total_responses = 0;

        for response in &self.responses {
            if let Some(value) = response.get(column) {
                if !value.trim().is_empty() && value != "NA" {
                    *counts.entry(value.clone()).or_insert(0) += 1;
                    total_responses += 1;
                }
            }
        }

        // Sort by count (descending)
        let mut sorted_counts: Vec<(&String, &usize)> = counts.iter().collect();
        sorted_counts.sort_by(|a, b| b.1.cmp(a.1));

        println!("Total valid responses: {}", total_responses);
        println!();

        for (option, count) in sorted_counts {
            let percentage = (*count as f64 / total_responses as f64) * 100.0;
            println!("{}: {} ({:.1}%)", option, count, percentage);
        }

        Ok(())
    }

    fn display_multiple_choice_distribution(&self, column: &str) -> Result<()> {
        let mut counts: HashMap<String, usize> = HashMap::new();
        let mut total_responses = 0;

        for response in &self.responses {
            if let Some(value) = response.get(column) {
                if !value.trim().is_empty() && value != "NA" {
                    total_responses += 1;
                    // Split multiple choice answers by semicolon
                    for option in value.split(';') {
                        let trimmed_option = option.trim();
                        if !trimmed_option.is_empty() {
                            *counts.entry(trimmed_option.to_string()).or_insert(0) += 1;
                        }
                    }
                }
            }
        }

        // Sort by count (descending)
        let mut sorted_counts: Vec<(&String, &usize)> = counts.iter().collect();
        sorted_counts.sort_by(|a, b| b.1.cmp(a.1));

        println!("Total responses: {}", total_responses);
        println!("Note: Percentages are based on total responses, not total options selected");
        println!();

        for (option, count) in sorted_counts {
            let percentage = (*count as f64 / total_responses as f64) * 100.0;
            println!("{}: {} ({:.1}%)", option, count, percentage);
        }

        Ok(())
    }

    /// Get all unique options for a question (useful for subset creation)
    pub fn get_question_options(&self, column: &str) -> Result<Vec<String>> {
        if !self.questions.contains_key(column) {
            return Err(anyhow!("Column '{}' not found", column));
        }

        let question = &self.questions[column];
        let mut options = std::collections::HashSet::new();

        for response in &self.responses {
            if let Some(value) = response.get(column) {
                if !value.trim().is_empty() && value != "NA" {
                    match question.question_type {
                        QuestionType::SC | QuestionType::TE => {
                            options.insert(value.clone());
                        }
                        QuestionType::MC => {
                            for option in value.split(';') {
                                let trimmed_option = option.trim();
                                if !trimmed_option.is_empty() {
                                    options.insert(trimmed_option.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }

        let mut sorted_options: Vec<String> = options.into_iter().collect();
        sorted_options.sort();
        Ok(sorted_options)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_test_data() -> SurveyData {
        let mut questions = IndexMap::new();
        questions.insert(
            "Q1".to_string(),
            SurveyQuestion {
                column: "Q1".to_string(),
                question_text: "What is your age?".to_string(),
                question_type: QuestionType::SC,
            },
        );
        questions.insert(
            "Q2".to_string(),
            SurveyQuestion {
                column: "Q2".to_string(),
                question_text: "What languages do you use?".to_string(),
                question_type: QuestionType::MC,
            },
        );

        let mut responses = Vec::new();
        let mut response1 = IndexMap::new();
        response1.insert("Q1".to_string(), "25-34".to_string());
        response1.insert("Q2".to_string(), "Python;JavaScript;Rust".to_string());
        responses.push(response1);

        let mut response2 = IndexMap::new();
        response2.insert("Q1".to_string(), "35-44".to_string());
        response2.insert("Q2".to_string(), "Java;Python".to_string());
        responses.push(response2);

        let mut response3 = IndexMap::new();
        response3.insert("Q1".to_string(), "25-34".to_string());
        response3.insert("Q2".to_string(), "Python;Go".to_string());
        responses.push(response3);

        SurveyData {
            questions,
            responses,
        }
    }

    #[test]
    fn test_search_questions() {
        let data = create_test_data();
        let results = data.search_questions("What is your age");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].column, "Q1");
    }

    #[test]
    fn test_create_subset() {
        let data = create_test_data();

        // Test single choice subset
        let subset = data.create_subset("Q1", "25-34").unwrap();
        assert_eq!(subset.len(), 2);

        // Test multiple choice subset
        let subset = data.create_subset("Q2", "Python").unwrap();
        assert_eq!(subset.len(), 3); // All responses contain Python

        let subset = data.create_subset("Q2", "Rust").unwrap();
        assert_eq!(subset.len(), 1); // Only first response contains Rust
    }

    #[test]
    fn test_get_question_options() {
        let data = create_test_data();

        // Test single choice options
        let options = data.get_question_options("Q1").unwrap();
        assert_eq!(options.len(), 2);
        assert!(options.contains(&"25-34".to_string()));
        assert!(options.contains(&"35-44".to_string()));

        // Test multiple choice options
        let options = data.get_question_options("Q2").unwrap();
        assert_eq!(options.len(), 5);
        assert!(options.contains(&"Python".to_string()));
        assert!(options.contains(&"JavaScript".to_string()));
        assert!(options.contains(&"Rust".to_string()));
        assert!(options.contains(&"Java".to_string()));
        assert!(options.contains(&"Go".to_string()));
    }
}
