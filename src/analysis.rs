use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::survey::QuestionType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnswerDistribution {
    pub question_id: usize,
    pub question_text: String,
    pub question_type: QuestionType,
    pub distribution: HashMap<String, (usize, f64)>, // (count, percentage)
    pub total_responses: usize,
}

impl AnswerDistribution {
    /// Display the distribution in a formatted way
    pub fn display(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("Question {}: {}\n", self.question_id, self.question_text));
        output.push_str(&format!("Type: {:?}\n", self.question_type));
        output.push_str(&format!("Total Responses: {}\n", self.total_responses));
        output.push_str("Distribution:\n");

        // Sort by count (descending)
        let mut items: Vec<_> = self.distribution.iter().collect();
        items.sort_by(|a, b| b.1.0.cmp(&a.1.0));

        for (option, (count, percentage)) in items {
            output.push_str(&format!("  {}: {} ({:.1}%)\n", option, count, percentage));
        }

        output
    }

    /// Get the most popular answer
    pub fn most_popular(&self) -> Option<(String, usize, f64)> {
        self.distribution
            .iter()
            .max_by_key(|(_, (count, _))| *count)
            .map(|(option, (count, percentage))| (option.clone(), *count, *percentage))
    }

    /// Get answers with percentage above threshold
    pub fn above_threshold(&self, threshold: f64) -> Vec<(String, usize, f64)> {
        self.distribution
            .iter()
            .filter(|(_, (_, percentage))| *percentage >= threshold)
            .map(|(option, (count, percentage))| (option.clone(), *count, *percentage))
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subset {
    pub question_id: usize,
    pub option: String,
    pub respondent_ids: Vec<usize>,
    pub total_respondents: usize,
}

impl Subset {
    /// Get the size of this subset
    pub fn size(&self) -> usize {
        self.respondent_ids.len()
    }

    /// Get the percentage of total respondents in this subset
    pub fn percentage(&self) -> f64 {
        if self.total_respondents > 0 {
            (self.size() as f64 / self.total_respondents as f64) * 100.0
        } else {
            0.0
        }
    }

    /// Display subset information
    pub fn display(&self) -> String {
        format!(
            "Subset for Question {} - Option '{}'\n\
            Size: {} respondents ({:.1}% of total)\n\
            Respondent IDs: {:?}",
            self.question_id,
            self.option,
            self.size(),
            self.percentage(),
            &self.respondent_ids[..self.respondent_ids.len().min(10)] // Show first 10 IDs
        )
    }

    /// Check if a respondent is in this subset
    pub fn contains_respondent(&self, respondent_id: usize) -> bool {
        self.respondent_ids.contains(&respondent_id)
    }

    /// Intersect with another subset
    pub fn intersect(&self, other: &Subset) -> Vec<usize> {
        self.respondent_ids
            .iter()
            .filter(|id| other.respondent_ids.contains(id))
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer_distribution() {
        let mut distribution = HashMap::new();
        distribution.insert("Rust".to_string(), (150, 30.0));
        distribution.insert("Python".to_string(), (250, 50.0));
        distribution.insert("JavaScript".to_string(), (100, 20.0));

        let answer_dist = AnswerDistribution {
            question_id: 1,
            question_text: "Favorite programming language".to_string(),
            question_type: QuestionType::SingleChoice,
            distribution,
            total_responses: 500,
        };

        let most_popular = answer_dist.most_popular().unwrap();
        assert_eq!(most_popular.0, "Python");
        assert_eq!(most_popular.1, 250);

        let above_30 = answer_dist.above_threshold(30.0);
        assert_eq!(above_30.len(), 2); // Python and Rust
    }

    #[test]
    fn test_subset() {
        let subset = Subset {
            question_id: 1,
            option: "Rust".to_string(),
            respondent_ids: vec![1, 2, 3, 4, 5],
            total_respondents: 100,
        };

        assert_eq!(subset.size(), 5);
        assert_eq!(subset.percentage(), 5.0);
        assert!(subset.contains_respondent(3));
        assert!(!subset.contains_respondent(10));
    }

    #[test]
    fn test_subset_intersection() {
        let subset1 = Subset {
            question_id: 1,
            option: "Rust".to_string(),
            respondent_ids: vec![1, 2, 3, 4, 5],
            total_respondents: 100,
        };

        let subset2 = Subset {
            question_id: 2,
            option: "Senior".to_string(),
            respondent_ids: vec![3, 4, 5, 6, 7],
            total_respondents: 100,
        };

        let intersection = subset1.intersect(&subset2);
        assert_eq!(intersection, vec![3, 4, 5]);
    }
}