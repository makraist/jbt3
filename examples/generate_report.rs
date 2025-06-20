// Report Generator Example
// This example generates a comprehensive survey analysis report

use anyhow::Result;
use stack_overflow_survey_analyzer::SurveyData;
use std::fs::File;
use std::io::Write;

fn main() -> Result<()> {
    println!("=== Stack Overflow Survey Report Generator ===\n");

    let data = SurveyData::load_from_excel("so_2024_raw2.xlsx")?;

    // Generate report content
    let mut report = String::new();

    // Header
    report.push_str("# Stack Overflow Developer Survey 2024 - Analysis Report\n\n");
    report.push_str(&format!(
        "Generated on: {}\n\n",
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    ));

    // Executive Summary
    report.push_str("## Executive Summary\n\n");
    report.push_str(&format!(
        "- **Total Survey Questions**: {}\n",
        data.questions.len()
    ));
    report.push_str(&format!(
        "- **Total Responses Analyzed**: {}\n",
        data.responses.len()
    ));
    report.push_str("\n");

    // Key Findings
    report.push_str("## Key Findings\n\n");

    // AI Adoption
    let ai_users = data.create_subset("AISelect", "Yes")?;
    let ai_non_users = data.create_subset("AISelect", "No, and I don't plan to")?;
    let ai_adoption_rate =
        (ai_users.len() as f64 / (ai_users.len() + ai_non_users.len()) as f64) * 100.0;

    report.push_str("### 🤖 AI Tool Adoption\n");
    report.push_str(&format!(
        "- **AI Adoption Rate**: {:.1}% ({} out of {} developers)\n",
        ai_adoption_rate,
        ai_users.len(),
        ai_users.len() + ai_non_users.len()
    ));
    report.push_str(&format!("- **AI Users**: {} developers\n", ai_users.len()));
    report.push_str(&format!(
        "- **AI Non-users**: {} developers\n",
        ai_non_users.len()
    ));
    report.push_str("\n");

    // Programming Languages
    report.push_str("### 💻 Programming Language Popularity\n");
    let languages = vec![
        "Python",
        "JavaScript",
        "HTML/CSS",
        "Java",
        "TypeScript",
        "Rust",
    ];
    for lang in languages {
        let lang_users = data.create_subset("LanguageHaveWorkedWith", lang)?;
        let adoption_rate = (lang_users.len() as f64 / data.responses.len() as f64) * 100.0;
        report.push_str(&format!(
            "- **{}**: {:.1}% ({} developers)\n",
            lang,
            adoption_rate,
            lang_users.len()
        ));
    }
    report.push_str("\n");

    // Demographics
    report.push_str("### 👥 Developer Demographics\n");

    // Age distribution
    let age_groups = vec![
        "Under 18 years old",
        "18-24 years old",
        "25-34 years old",
        "35-44 years old",
        "45-54 years old",
    ];
    report.push_str("**Age Distribution:**\n");
    for age in age_groups {
        let age_subset = data.create_subset("Age", age)?;
        if !age_subset.is_empty() {
            let percentage = (age_subset.len() as f64 / data.responses.len() as f64) * 100.0;
            report.push_str(&format!(
                "- {}: {:.1}% ({} developers)\n",
                age,
                percentage,
                age_subset.len()
            ));
        }
    }
    report.push_str("\n");

    // Experience levels
    report.push_str("**Experience Levels:**\n");
    let mut experience_data = std::collections::HashMap::new();
    for response in &data.responses {
        if let Some(years) = response.get("YearsCode") {
            if !years.trim().is_empty() && years != "NA" {
                *experience_data.entry(years.clone()).or_insert(0) += 1;
            }
        }
    }

    for (years, count) in &experience_data {
        let percentage = (*count as f64 / data.responses.len() as f64) * 100.0;
        report.push_str(&format!(
            "- {}: {:.1}% ({} developers)\n",
            years, percentage, count
        ));
    }
    report.push_str("\n");

    // Technology Insights
    report.push_str("### 🔧 Technology Insights\n");

    // Database usage
    report.push_str("**Database Technologies:**\n");
    let databases = vec!["PostgreSQL", "MySQL", "SQLite", "MongoDB"];
    for db in databases {
        let db_users = data.create_subset("DatabaseHaveWorkedWith", db)?;
        if !db_users.is_empty() {
            let adoption_rate = (db_users.len() as f64 / data.responses.len() as f64) * 100.0;
            report.push_str(&format!(
                "- {}: {:.1}% ({} developers)\n",
                db,
                adoption_rate,
                db_users.len()
            ));
        }
    }
    report.push_str("\n");

    // Development Environment
    report.push_str("**Development Environments:**\n");
    let ides = vec!["Visual Studio Code", "Vim", "PyCharm", "IntelliJ"];
    for ide in ides {
        let ide_users = data.create_subset("NEWCollabToolsHaveWorkedWith", ide)?;
        if !ide_users.is_empty() {
            let adoption_rate = (ide_users.len() as f64 / data.responses.len() as f64) * 100.0;
            report.push_str(&format!(
                "- {}: {:.1}% ({} developers)\n",
                ide,
                adoption_rate,
                ide_users.len()
            ));
        }
    }
    report.push_str("\n");

    // Stack Overflow Usage
    report.push_str("### 📚 Stack Overflow Usage Patterns\n");
    let mut so_usage = std::collections::HashMap::new();
    for response in &data.responses {
        if let Some(frequency) = response.get("SOVisitFreq") {
            if !frequency.trim().is_empty() && frequency != "NA" {
                *so_usage.entry(frequency.clone()).or_insert(0) += 1;
            }
        }
    }

    for (frequency, count) in &so_usage {
        let percentage = (*count as f64 / data.responses.len() as f64) * 100.0;
        report.push_str(&format!(
            "- {}: {:.1}% ({} developers)\n",
            frequency, percentage, count
        ));
    }
    report.push_str("\n");

    // Interesting Correlations
    report.push_str("## Interesting Correlations\n\n");

    // AI users vs Rust adoption
    let rust_ai_users = ai_users
        .iter()
        .filter(|dev| {
            dev.get("LanguageHaveWorkedWith")
                .map(|langs| langs.contains("Rust"))
                .unwrap_or(false)
        })
        .count();

    let rust_non_ai_users = ai_non_users
        .iter()
        .filter(|dev| {
            dev.get("LanguageHaveWorkedWith")
                .map(|langs| langs.contains("Rust"))
                .unwrap_or(false)
        })
        .count();

    report.push_str("### 🦀 Rust and AI Tool Usage\n");
    if !ai_users.is_empty() && !ai_non_users.is_empty() {
        let rust_ai_pct = (rust_ai_users as f64 / ai_users.len() as f64) * 100.0;
        let rust_non_ai_pct = (rust_non_ai_users as f64 / ai_non_users.len() as f64) * 100.0;
        report.push_str(&format!(
            "- Rust usage among AI users: {:.1}% ({}/{})\n",
            rust_ai_pct,
            rust_ai_users,
            ai_users.len()
        ));
        report.push_str(&format!(
            "- Rust usage among AI non-users: {:.1}% ({}/{})\n",
            rust_non_ai_pct,
            rust_non_ai_users,
            ai_non_users.len()
        ));
    }
    report.push_str("\n");

    // Recommendations
    report.push_str("## Recommendations\n\n");
    report.push_str("1. **AI Integration**: With ");
    report.push_str(&format!("{:.1}% adoption rate, ", ai_adoption_rate));
    report.push_str("AI tools are becoming mainstream in development workflows.\n");
    report.push_str("2. **Language Trends**: JavaScript, Python, and HTML/CSS remain dominant, ");
    report.push_str("while Rust shows growing adoption among early adopters.\n");
    report.push_str(
        "3. **Developer Education**: The mix of experience levels suggests opportunities ",
    );
    report.push_str("for mentorship and knowledge sharing programs.\n");
    report.push_str(
        "4. **Tool Ecosystem**: Developers use diverse toolsets, indicating the importance ",
    );
    report.push_str("of interoperability and standardization.\n\n");

    // Footer
    report.push_str("---\n");
    report.push_str(
        "*This report was generated using the Stack Overflow Survey Analyzer library.*\n",
    );

    // Write report to file
    let mut file = File::create("survey_analysis_report.md")?;
    file.write_all(report.as_bytes())?;

    println!("✅ Report generated successfully!");
    println!("📄 Report saved as: survey_analysis_report.md");
    println!("📊 Report contains {} sections with detailed analysis", 6);

    // Also print a summary to console
    println!("\n=== QUICK SUMMARY ===");
    println!(
        "📊 Survey Overview: {} questions, {} responses",
        data.questions.len(),
        data.responses.len()
    );
    println!(
        "🤖 AI Adoption: {:.1}% of developers use AI tools",
        ai_adoption_rate
    );
    println!("💻 Top Languages: Python, JavaScript, HTML/CSS dominate");
    println!("📚 Stack Overflow: Developers actively engage with the platform");

    Ok(())
}
