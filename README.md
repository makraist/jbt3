# Stack Overflow Survey Analyzer

A Rust library and CLI tool for analyzing Stack Overflow Survey data. This library provides functionality to analyze the 2024 Stack Overflow Developer Survey data, including displaying survey structure, searching questions, creating respondent subsets, and showing answer distributions.

## Features

- 📊 **Survey Structure Display**: View all survey questions with their types (Single Choice, Multiple Choice, Text Entry)
- 🔍 **Question Search**: Search for questions by keyword in question text or column names
- 🎯 **Respondent Subsets**: Create filtered subsets of respondents based on specific question responses
- 📈 **Distribution Analysis**: Display answer distributions with percentages for both single-choice and multiple-choice questions
- 💻 **CLI Interface**: Command-line tool with both one-off commands and interactive REPL mode
- 🧪 **Unit Tests**: Comprehensive test coverage for core functionality

## Installation

### Prerequisites

- Rust 1.70+ (with Cargo)
- The Stack Overflow survey data file (`so_2024_raw2.xlsx`)

### Setup

1. Clone or download this project
2. Navigate to the project directory:
   ```bash
   cd stack_overflow_survey_analyzer
   ```

3. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

### CLI Tool

The project includes a command-line interface (`so_analyzer`) with several commands:

#### Basic Commands

**Display Survey Structure:**
```bash
cargo run --bin so_analyzer structure
```

**Search for Questions:**
```bash
cargo run --bin so_analyzer search "programming language"
cargo run --bin so_analyzer search "AI"
```

**Show Answer Distribution:**
```bash
cargo run --bin so_analyzer distribution Age
cargo run --bin so_analyzer distribution LanguageHaveWorkedWith
```

**Create Respondent Subset:**
```bash
cargo run --bin so_analyzer subset Age "25-34 years old"
cargo run --bin so_analyzer subset --detailed LanguageHaveWorkedWith "Rust"
```

**Get Available Options for a Question:**
```bash
cargo run --bin so_analyzer options Age
cargo run --bin so_analyzer options Employment
```

#### Interactive Mode

For exploration and analysis, use the interactive REPL mode:

```bash
cargo run --bin so_analyzer interactive
```

In interactive mode, you can use these commands:
- `structure` - Display survey structure
- `search <keyword>` - Search for questions
- `dist <column>` - Show distribution for a question
- `subset <column> <option>` - Create subset of respondents
- `options <column>` - Show available options for a question
- `columns` - List all column names
- `help` - Show available commands
- `quit` - Exit interactive mode

#### Custom Data File

By default, the tool looks for `so_2024_raw2.xlsx` in the current directory. You can specify a different file:

```bash
cargo run --bin so_analyzer -f /path/to/your/survey_data.xlsx structure
```

### Library Usage

You can also use this as a library in your own Rust projects:

```toml
[dependencies]
stack_overflow_survey_analyzer = { path = "path/to/this/project" }
```

```rust
use stack_overflow_survey_analyzer::SurveyData;

fn main() -> anyhow::Result<()> {
    // Load survey data
    let data = SurveyData::load_from_excel("so_2024_raw2.xlsx")?;
    
    // Display survey structure
    data.display_survey_structure();
    
    // Search for questions
    let results = data.search_questions("language");
    println!("Found {} questions about languages", results.len());
    
    // Create subset of Python developers
    let python_devs = data.create_subset("LanguageHaveWorkedWith", "Python")?;
    println!("Found {} Python developers", python_devs.len());
    
    // Show age distribution
    data.display_distribution("Age")?;
    
    Ok(())
}
```

## Data Structure

The survey data consists of two sheets:

1. **raw data**: Contains actual survey responses with one row per respondent
2. **schema**: Contains metadata about each question including:
   - `column`: Column name in the data
   - `question_text`: The actual survey question
   - `type`: Question type (SC = Single Choice, MC = Multiple Choice, TE = Text Entry)

## Examples

### Example 1: Analyzing Programming Languages

```bash
# Find all language-related questions
cargo run --bin so_analyzer search "language"

# Show distribution of languages people have worked with
cargo run --bin so_analyzer distribution LanguageHaveWorkedWith

# Find Rust developers
cargo run --bin so_analyzer subset LanguageHaveWorkedWith "Rust"
```

### Example 2: Age Demographics Analysis

```bash
# Show age distribution
cargo run --bin so_analyzer distribution Age

# Find all available age options
cargo run --bin so_analyzer options Age

# Create subset of developers aged 25-34
cargo run --bin so_analyzer subset Age "25-34 years old"
```

### Example 3: AI Tool Usage Analysis

```bash
# Search for AI-related questions
cargo run --bin so_analyzer search "AI"

# Show distribution of AI tool usage
cargo run --bin so_analyzer distribution AISelect

# Find developers using AI tools
cargo run --bin so_analyzer subset AISelect "Yes"
```

## Running Tests

Run the unit tests to ensure everything is working correctly:

```bash
cargo test
```

Run tests with output:
```bash
cargo test -- --nocapture
```

Run specific test:
```bash
cargo test test_search_questions
```

## API Reference

### Core Types

- `SurveyData`: Main struct containing survey questions and responses
- `SurveyQuestion`: Represents a single survey question with metadata
- `QuestionType`: Enum for question types (SC, MC, TE)

### Main Functions

- `SurveyData::load_from_excel(path)`: Load survey data from Excel file
- `display_survey_structure()`: Print all questions and their types
- `search_questions(keyword)`: Find questions matching a keyword
- `create_subset(column, option)`: Filter respondents by answer
- `display_distribution(column)`: Show answer distribution with percentages
- `get_question_options(column)`: Get all unique answers for a question

## Error Handling

The library uses `anyhow::Result` for error handling. Common errors include:

- File not found or invalid Excel format
- Column not found in the dataset
- Invalid question types or malformed data

## Performance Notes

- The library loads all data into memory for fast analysis
- Large datasets (>100MB) may require significant RAM
- Distribution calculations are cached per session
- Interactive mode keeps data in memory for responsive queries

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass with `cargo test`
5. Submit a pull request

## License

This project is provided as-is for educational and analysis purposes.

## Data Source

This library is designed to work with Stack Overflow Developer Survey data. The 2024 survey data should be obtained from the official Stack Overflow Developer Survey website.
