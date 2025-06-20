// Advanced Analysis Example
// This example demonstrates sophisticated analysis workflows and cross-tabulation

use anyhow::Result;
use stack_overflow_survey_analyzer::SurveyData;
use std::collections::HashMap;

fn main() -> Result<()> {
    println!("=== Stack Overflow Survey Analyzer - Advanced Analysis ===\n");

    let data = SurveyData::load_from_excel("so_2024_raw2.xlsx")?;
    println!(
        "Data loaded: {} questions, {} responses\n",
        data.questions.len(),
        data.responses.len()
    );

    // 1. AI Adoption Analysis
    println!("🤖 AI ADOPTION ANALYSIS");
    println!("=".repeat(50));

    // Get AI users and non-users
    let ai_users = data.create_subset("AISelect", "Yes")?;
    let ai_non_users = data.create_subset("AISelect", "No, and I don't plan to")?;

    println!("AI Users: {} respondents", ai_users.len());
    println!("AI Non-users: {} respondents", ai_non_users.len());
    println!();

    // 2. Cross-tabulation: AI Usage by Age Group
    println!("📊 AI Usage by Age Group:");
    let mut age_ai_cross_tab = HashMap::new();

    for response in &data.responses {
        if let (Some(age), Some(ai_usage)) = (response.get("Age"), response.get("AISelect")) {
            if !age.trim().is_empty()
                && age != "NA"
                && !ai_usage.trim().is_empty()
                && ai_usage != "NA"
            {
                let entry = age_ai_cross_tab
                    .entry(age.clone())
                    .or_insert_with(HashMap::new);
                *entry.entry(ai_usage.clone()).or_insert(0) += 1;
            }
        }
    }

    for (age, ai_usage_counts) in &age_ai_cross_tab {
        println!("  {}: ", age);
        for (usage, count) in ai_usage_counts {
            println!("    {}: {}", usage, count);
        }
    }
    println!();

    // 3. Technology Stack Analysis
    println!("💻 TECHNOLOGY STACK ANALYSIS");
    println!("=".repeat(50));

    // Find developers using modern vs traditional languages
    let rust_devs = data.create_subset("LanguageHaveWorkedWith", "Rust")?;
    let python_devs = data.create_subset("LanguageHaveWorkedWith", "Python")?;
    let js_devs = data.create_subset("LanguageHaveWorkedWith", "JavaScript")?;

    println!("Language Adoption:");
    println!("  Rust developers: {}", rust_devs.len());
    println!("  Python developers: {}", python_devs.len());
    println!("  JavaScript developers: {}", js_devs.len());
    println!();

    // 4. Developer Profile Analysis
    println!("👥 DEVELOPER PROFILE ANALYSIS");
    println!("=".repeat(50));

    // Analyze Rust developers in detail
    if !rust_devs.is_empty() {
        println!("🦀 Rust Developer Profiles:");
        for (i, dev) in rust_devs.iter().enumerate() {
            println!("  Developer {}:", i + 1);
            if let Some(age) = dev.get("Age") {
                println!("    Age: {}", age);
            }
            if let Some(exp) = dev.get("YearsCode") {
                println!("    Years coding: {}", exp);
            }
            if let Some(ai_usage) = dev.get("AISelect") {
                println!("    Uses AI: {}", ai_usage);
            }
            if let Some(education) = dev.get("EdLevel") {
                println!("    Education: {}", education);
            }
            println!();
        }
    }

    // 5. Search pattern analysis
    println!("🔍 SEARCH PATTERN ANALYSIS");
    println!("=".repeat(50));

    let search_terms = vec!["AI", "language", "database", "framework", "tool"];
    for term in search_terms {
        let results = data.search_questions(term);
        println!("  '{}' appears in {} questions", term, results.len());
    }
    println!();

    // 6. Experience Level Analysis
    println!("📈 EXPERIENCE LEVEL ANALYSIS");
    println!("=".repeat(50));

    // Analyze coding experience distribution
    data.display_distribution("YearsCode")?;
    println!();

    // 7. Education vs Experience Cross-analysis
    println!("🎓 EDUCATION vs EXPERIENCE");
    println!("=".repeat(50));

    let mut edu_exp_cross = HashMap::new();
    for response in &data.responses {
        if let (Some(education), Some(experience)) =
            (response.get("EdLevel"), response.get("YearsCode"))
        {
            if !education.trim().is_empty()
                && education != "NA"
                && !experience.trim().is_empty()
                && experience != "NA"
            {
                let entry = edu_exp_cross
                    .entry(education.clone())
                    .or_insert_with(Vec::new);
                entry.push(experience.clone());
            }
        }
    }

    for (education, experiences) in &edu_exp_cross {
        println!("  {}: {} respondents", education, experiences.len());
        println!("    Experience levels: {:?}", experiences);
    }

    println!("\n✨ Advanced analysis complete!");
    Ok(())
}
