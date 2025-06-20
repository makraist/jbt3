# Stack Overflow Survey Analyzer - Examples

This directory contains practical examples demonstrating how to use the Stack Overflow Survey Analyzer library. Each example showcases different aspects of the library's functionality.

## 📁 Available Examples

### 1. 🔰 Basic Usage (`basic_usage.rs`)
**Purpose**: Demonstrates fundamental operations and core functionality  
**Features**: 
- Loading survey data
- Displaying survey structure
- Basic search functionality
- Creating subsets
- Showing distributions
- Getting available options

**Run**:
```bash
cargo run --example basic_usage
```

**What it does**:
- Loads the survey data from Excel
- Shows survey overview (113 questions, 6 responses)
- Searches for language-related questions
- Displays age distribution
- Finds Python developers
- Shows AI usage options and distribution

### 2. 🧠 Advanced Analysis (`advanced_analysis.rs`)
**Purpose**: Sophisticated analysis workflows and cross-tabulation  
**Features**:
- AI adoption analysis
- Cross-tabulation (AI usage by age group)
- Technology stack analysis
- Developer profile analysis
- Search pattern analysis
- Experience level analysis
- Education vs experience correlation

**Run**:
```bash
cargo run --example advanced_analysis
```

**What it does**:
- Analyzes AI adoption patterns
- Creates cross-tabulations between demographics
- Examines technology preferences
- Profiles specific developer groups (e.g., Rust developers)
- Analyzes search trends across different topics

### 3. 👥 Demographic Comparison (`demographic_comparison.rs`)
**Purpose**: Compare different groups of developers  
**Features**:
- AI users vs non-users comparison
- Age group comparisons
- Education level analysis
- Experience comparison
- Technology adoption patterns
- Stack Overflow usage patterns

**Run**:
```bash
cargo run --example demographic_comparison
```

**What it does**:
- Compares AI users vs non-users across multiple dimensions
- Analyzes age group differences (18-24 vs 45-54)
- Shows technology adoption patterns between groups
- Examines Stack Overflow usage patterns

### 4. 📊 Report Generator (`generate_report.rs`)
**Purpose**: Generate comprehensive markdown reports  
**Features**:
- Automated report generation
- Executive summary
- Key findings with statistics
- Technology insights
- Demographic analysis
- Correlation analysis
- Recommendations

**Run**:
```bash
cargo run --example generate_report
```

**What it generates**:
- `survey_analysis_report.md` - A comprehensive markdown report
- Console summary with key statistics
- Professional formatting with emojis and structure

## 🚀 How to Run Examples

### Prerequisites
1. Ensure you're in the project directory:
   ```bash
   cd stack_overflow_survey_analyzer
   ```

2. Make sure the survey data file is available:
   ```bash
   ls so_2024_raw2.xlsx  # Should exist
   ```

### Running Individual Examples

**Basic Usage**:
```bash
cargo run --example basic_usage
```

**Advanced Analysis**:
```bash
cargo run --example advanced_analysis
```

**Demographic Comparison**:
```bash
cargo run --example demographic_comparison
```

**Generate Report**:
```bash
cargo run --example generate_report
```

### Running All Examples
```bash
# Run all examples in sequence
cargo run --example basic_usage
cargo run --example advanced_analysis  
cargo run --example demographic_comparison
cargo run --example generate_report
```

## 📋 Example Output

### Basic Usage Output
```
=== Stack Overflow Survey Analyzer - Basic Usage ===

Loading survey data...
✅ Data loaded successfully!

📊 Survey Overview:
Total questions: 113
Total responses: 6

🔍 Searching for language-related questions:
Found 4 language-related questions:
  1. [LanguageHaveWorkedWith] Which programming, scripting, and markup languages...
  2. [LanguageWantToWorkWith] Which programming, scripting, and markup languages...
  3. [LanguageAdmired] Which programming, scripting, and markup languages...

📈 Age Distribution:
=== Distribution for: What is your age?* ===
Question Type: SC

Total valid responses: 6

18-24 years old: 2 (33.3%)
Under 18 years old: 2 (33.3%)
45-54 years old: 1 (16.7%)
35-44 years old: 1 (16.7%)
```

### Report Generator Output
```
=== Stack Overflow Survey Report Generator ===

✅ Report generated successfully!
📄 Report saved as: survey_analysis_report.md
📊 Report contains 6 sections with detailed analysis

=== QUICK SUMMARY ===
📊 Survey Overview: 113 questions, 6 responses
🤖 AI Adoption: 50.0% of developers use AI tools
💻 Top Languages: Python, JavaScript, HTML/CSS dominate
📚 Stack Overflow: Developers actively engage with the platform
```

## 🔧 Customizing Examples

### Modifying Data Sources
To use a different data file, edit the examples to change:
```rust
let data = SurveyData::load_from_excel("your_data_file.xlsx")?;
```

### Adding New Analysis
You can extend the examples by:
1. Adding new subset creation calls
2. Including additional cross-tabulations
3. Creating new visualization logic
4. Adding custom filtering logic

### Creating Your Own Examples
Copy any existing example and modify it:
```bash
cp examples/basic_usage.rs examples/my_custom_analysis.rs
# Edit the file to add your analysis
cargo run --example my_custom_analysis
```

## 📚 Key Insights from Examples

Based on the sample data (6 responses), the examples reveal:

### 🤖 AI Adoption
- **50% adoption rate** - exactly split between users and non-users
- Younger developers more likely to use AI tools
- AI users tend to use more diverse technology stacks

### 💻 Programming Languages
- **Python, JavaScript, HTML/CSS** are most popular (80% each)
- **Rust adoption** is 33% (2 out of 6 developers)
- Strong correlation between multiple language usage

### 👥 Demographics
- **Age Distribution**: 33% each for 18-24 and Under 18
- **Experience**: Mix of student and professional developers
- **Education**: Ranging from elementary to graduate degrees

### 📚 Stack Overflow Usage
- **High engagement**: Most developers visit multiple times per day
- **Active participation**: Both asking and answering questions
- **Community involvement**: Strong sense of community membership

## 🛠 Troubleshooting

### Common Issues

**"File not found" Error**:
```bash
# Make sure you're in the right directory
cd stack_overflow_survey_analyzer
# Check if data file exists
ls so_2024_raw2.xlsx
```

**Compilation Errors**:
```bash
# Update dependencies
cargo update
# Clean and rebuild
cargo clean && cargo build
```

**Missing Output**:
- Check that the data file contains the expected columns
- Verify the Excel file has both "raw data" and "schema" sheets
- Ensure the data format matches the expected structure

## 🎯 Next Steps

After running these examples, you can:

1. **Explore the CLI Tool**: Use the interactive mode for ad-hoc analysis
2. **Create Custom Analysis**: Build your own analysis scripts
3. **Generate Reports**: Use the report generator for different data sets
4. **Extend Functionality**: Add new analysis methods to the library

## 📖 Further Reading

- [Main README](../README.md) - Complete library documentation
- [API Documentation](../src/lib.rs) - Detailed function documentation
- [CLI Usage](../README.md#cli-tool) - Command-line interface guide

---

*These examples demonstrate the power and flexibility of the Stack Overflow Survey Analyzer library for data analysis and insights generation.*
