# Stack Overflow Survey Analyzer

A Rust library for analyzing Stack Overflow Survey data with CLI and REPL interfaces. This library provides tools to explore survey structure, search questions and options, create respondent subsets, and analyze answer distributions.

## Features

- **Load Excel survey data** - Read Stack Overflow survey data from Excel files (.xlsx format)
- **Survey structure exploration** - List and filter questions by type and content
- **Search functionality** - Find questions and answer options by keywords
- **Respondent subsetting** - Create subsets of respondents based on specific answers
- **Answer distribution analysis** - Display frequency distributions for single-choice and multiple-choice questions
- **CLI interface** - Command-line tools for all operations
- **Interactive REPL** - Real-time exploration of survey data
- **Comprehensive testing** - Unit tests for all core functionality

## Installation & Setup

### Prerequisites

- Rust 1.70+ 
- Cargo

### Setup

1. **Navigate to the project directory:**
   ```bash
   cd /Users/lwoznicki/Documents/exp/so_survey_analyzer
   ```

2. **Build the project:**
   ```bash
   cargo build --release
   ```

3. **Ensure the survey data file is accessible:**
   - The default path is `../so_2024_raw.xlsx` (relative to the binary location)
   - You can specify a different path using the `--file` option

## Usage

### Library Usage

```rust
use so_survey_analyzer::{SurveyAnalyzer, SurveyError};

fn main() -> Result<(), SurveyError> {
    // Load survey data
    let analyzer = SurveyAnalyzer::from_excel("path/to/so_2024_raw.xlsx")?;
    
    // Get survey structure
    let questions = analyzer.get_survey_structure();
    println!("Found {} questions", questions.len());
    
    // Search for questions
    let rust_questions = analyzer.search_questions("rust");
    
    // Create a subset
    let rust_developers = analyzer.create_subset(5, "Rust")?;
    println!("Found {} Rust developers", rust_developers.size());
    
    // Get answer distribution
    let distribution = analyzer.get_distribution(10)?;
    println!("{}", distribution.display());
    
    Ok(())
}
```

### CLI Usage

The library includes a command-line interface with several subcommands:

#### Display Survey Structure
```bash
# Show all questions
cargo run --bin so_survey_cli structure

# Show only first 10 questions
cargo run --bin so_survey_cli structure --limit 10

# Show questions containing "programming"
cargo run --bin so_survey_cli structure --filter programming
```

#### Search Questions and Options
```bash
# Search for questions containing "language"
cargo run --bin so_survey_cli search "programming language"

# Search for answer options containing "rust"
cargo run --bin so_survey_cli search "rust" --options
```

#### Create Respondent Subsets
```bash
# Create subset of respondents who answered "Rust" to question 5
cargo run --bin so_survey_cli subset 5 "Rust"
```

#### Display Answer Distributions
```bash
# Show distribution for question 10
cargo run --bin so_survey_cli distribution 10

# Show only answers with at least 5% share
cargo run --bin so_survey_cli distribution 10 --threshold 5.0
```

#### Interactive REPL Mode
```bash
# Start interactive mode
cargo run --bin so_survey_cli repl
```

In REPL mode, you can use these commands:
- `list [limit]` - List questions (optionally limit to N questions)
- `search <term>` - Search questions containing term
- `searchopt <term>` - Search answer options containing term
- `dist <question_id>` - Show answer distribution for question
- `subset <question_id> <option>` - Create respondent subset
- `help` - Show available commands
- `quit` - Exit REPL

### Custom File Path

```bash
# Use a different survey data file
cargo run --bin so_survey_cli --file /path/to/survey.xlsx structure
```

## API Reference

### Core Types

#### `SurveyAnalyzer`
Main entry point for the library.

**Methods:**
- `from_excel(path)` - Load survey data from Excel file
- `get_survey_structure()` - Get all questions
- `search_questions(term)` - Search questions by keyword
- `search_options(term)` - Search answer options by keyword
- `create_subset(question_id, option)` - Create respondent subset
- `get_distribution(question_id)` - Get answer distribution

#### `Question`
Represents a survey question.

**Fields:**
- `id: usize` - Question identifier
- `text: String` - Question text
- `question_type: QuestionType` - Type of question
- `options: Vec<String>` - Available answer options

#### `QuestionType`
Enum representing question types:
- `SingleChoice` - Single-choice questions
- `MultipleChoice` - Multiple-choice questions
- `Text` - Text/open-ended questions
- `Numeric` - Numeric questions

#### `AnswerDistribution`
Contains distribution analysis for a question.

**Methods:**
- `display()` - Format distribution for display
- `most_popular()` - Get most frequent answer
- `above_threshold(threshold)` - Get answers above percentage threshold

#### `Subset`
Represents a subset of respondents.

**Methods:**
- `size()` - Number of respondents in subset
- `percentage()` - Percentage of total respondents
- `display()` - Format subset information
- `contains_respondent(id)` - Check if respondent is in subset
- `intersect(other)` - Intersect with another subset

## Data Format

The library expects Excel files (.xlsx) with:
- **First row**: Column headers (question text)
- **Subsequent rows**: Survey responses
- **Multiple choice answers**: Separated by semicolons (`;`) or commas (`,`)
- **Missing values**: Empty cells or "NA"

Example format:
```
| What is your role? | Languages used          | Years of experience |
|--------------------|-------------------------|---------------------|
| Developer          | Python;JavaScript;Rust  | 5                   |
| Manager            | Python                  | 10                  |
| Student            | Rust;C++                | 1                   |
```

## Running Tests

Run the complete test suite:
```bash
cargo test
```

Run specific test modules:
```bash
# Test survey functionality
cargo test survey

# Test analysis functionality  
cargo test analysis

# Test error handling
cargo test error
```

Run tests with output:
```bash
cargo test -- --nocapture
```

## Project Structure

```
src/
├── lib.rs              # Main library interface
├── error.rs            # Error types and handling
├── survey.rs           # Survey data structures and Excel loading
├── analysis.rs         # Distribution and subset analysis
└── bin/
    └── cli.rs          # Command-line interface
```

## Dependencies

- **calamine** - Excel file reading
- **serde** - Serialization support
- **clap** - Command-line argument parsing
- **thiserror** - Error handling
- **anyhow** - Error context

## Error Handling

The library uses a comprehensive error system with specific error types:
- `SurveyError::Io` - File I/O errors
- `SurveyError::ExcelError` - Excel parsing errors
- `SurveyError::QuestionNotFound` - Invalid question IDs
- `SurveyError::InvalidQuestionType` - Unsupported operations
- `SurveyError::OptionNotFound` - Invalid answer options
- `SurveyError::DataParsingError` - Data format issues
- `SurveyError::EmptyDataset` - Empty or invalid datasets

## Performance Considerations

- **Memory usage**: The library loads the entire dataset into memory for fast querying
- **Excel parsing**: Initial loading may take time for large files (the so_2024_raw.xlsx is ~36MB)
- **Search operations**: Text searching is case-insensitive but not indexed
- **Subset operations**: Efficient filtering using iterators

## Examples

### Basic Analysis Workflow

```rust
use so_survey_analyzer::SurveyAnalyzer;

fn analyze_programming_languages() -> Result<(), Box<dyn std::error::Error>> {
    let analyzer = SurveyAnalyzer::from_excel("so_2024_raw.xlsx")?;
    
    // Find programming language question
    let lang_questions = analyzer.search_questions("programming language");
    if let Some(question) = lang_questions.first() {
        println!("Analyzing: {}", question.text);
        
        // Get distribution
        let dist = analyzer.get_distribution(question.id)?;
        if let Some((most_popular, count, percentage)) = dist.most_popular() {
            println!("Most popular: {} ({} responses, {:.1}%)", 
                     most_popular, count, percentage);
        }
        
        // Create subset for Rust developers
        let rust_devs = analyzer.create_subset(question.id, "Rust")?;
        println!("Rust developers: {} ({:.1}%)", 
                 rust_devs.size(), rust_devs.percentage());
    }
    
    Ok(())
}
```

### Subset Intersection Analysis

```rust
use so_survey_analyzer::SurveyAnalyzer;

fn analyze_rust_senior_developers() -> Result<(), Box<dyn std::error::Error>> {
    let analyzer = SurveyAnalyzer::from_excel("so_2024_raw.xlsx")?;
    
    // Create subsets
    let rust_devs = analyzer.create_subset(5, "Rust")?;
    let senior_devs = analyzer.create_subset(10, "Senior")?;
    
    // Find intersection
    let rust_seniors = rust_devs.intersect(&senior_devs);
    println!("Senior Rust developers: {} respondents", rust_seniors.len());
    
    Ok(())
}
```

## License

MIT