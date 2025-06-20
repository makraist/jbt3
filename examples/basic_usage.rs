// Basic Usage Example
// This example demonstrates the fundamental operations of the Stack Overflow Survey Analyzer

use anyhow::Result;
use stack_overflow_survey_analyzer::SurveyData;

fn main() -> Result<()> {
    println!("=== Stack Overflow Survey Analyzer - Basic Usage ===\n");

    // Load the survey data
    println!("Loading survey data...");
    let data = SurveyData::load_from_excel("so_2024_raw2.xlsx")?;
    println!("✅ Data loaded successfully!\n");

    // 1. Display basic survey information
    println!("📊 Survey Overview:");
    println!("Total questions: {}", data.questions.len());
    println!("Total responses: {}\n", data.responses.len());

    // 2. Search for questions about programming languages
    println!("🔍 Searching for language-related questions:");
    let language_questions = data.search_questions("language");
    println!(
        "Found {} language-related questions:",
        language_questions.len()
    );
    for (i, q) in language_questions.iter().take(3).enumerate() {
        println!("  {}. [{}] {}", i + 1, q.column, q.question_text);
    }
    println!();

    // 3. Show age distribution
    println!("📈 Age Distribution:");
    data.display_distribution("Age")?;
    println!();

    // 4. Find Python developers
    println!("🐍 Finding Python developers:");
    let python_devs = data.create_subset("LanguageHaveWorkedWith", "Python")?;
    println!("Found {} Python developers", python_devs.len());
    println!();

    // 5. Show available options for AI usage
    println!("🤖 AI Tool Usage Options:");
    let ai_options = data.get_question_options("AISelect")?;
    for (i, option) in ai_options.iter().enumerate() {
        println!("  {}. {}", i + 1, option);
    }
    println!();

    // 6. Compare AI usage
    println!("🤖 AI Tool Usage Distribution:");
    data.display_distribution("AISelect")?;

    println!("\n✨ Basic analysis complete!");
    Ok(())
}
