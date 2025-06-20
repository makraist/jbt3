// Demographic Comparison Example
// This example demonstrates how to compare different groups of developers

use anyhow::Result;
use stack_overflow_survey_analyzer::SurveyData;

fn main() -> Result<()> {
    println!("=== Demographics Comparison Analysis ===\n");

    let data = SurveyData::load_from_excel("so_2024_raw2.xlsx")?;

    // Compare AI adopters vs non-adopters
    println!("🤖 AI ADOPTERS vs NON-ADOPTERS COMPARISON");
    println!("=".repeat(60));

    let ai_users = data.create_subset("AISelect", "Yes")?;
    let ai_non_users = data.create_subset("AISelect", "No, and I don't plan to")?;

    println!("Sample sizes:");
    println!("  AI Users: {}", ai_users.len());
    println!("  AI Non-users: {}", ai_non_users.len());
    println!();

    // Age distribution comparison
    println!("👥 Age Distribution Comparison:");
    analyze_group_demographics("AI Users", &ai_users);
    analyze_group_demographics("AI Non-users", &ai_non_users);
    println!();

    // Education level comparison
    println!("🎓 Education Level Comparison:");
    analyze_group_education("AI Users", &ai_users);
    analyze_group_education("AI Non-users", &ai_non_users);
    println!();

    // Experience level comparison
    println!("💼 Experience Level Comparison:");
    analyze_group_experience("AI Users", &ai_users);
    analyze_group_experience("AI Non-users", &ai_non_users);
    println!();

    // Programming language preferences
    println!("💻 Programming Language Preferences:");
    analyze_group_languages("AI Users", &ai_users);
    analyze_group_languages("AI Non-users", &ai_non_users);
    println!();

    // Stack Overflow usage patterns
    println!("📚 Stack Overflow Usage Patterns:");
    analyze_so_usage("AI Users", &ai_users);
    analyze_so_usage("AI Non-users", &ai_non_users);
    println!();

    // Compare different age groups
    println!("🔍 AGE GROUP COMPARISON");
    println!("=".repeat(60));

    let young_devs = data.create_subset("Age", "18-24 years old")?;
    let older_devs = data.create_subset("Age", "45-54 years old")?;

    println!("Young Developers (18-24): {}", young_devs.len());
    println!("Older Developers (45-54): {}", older_devs.len());
    println!();

    if !young_devs.is_empty() && !older_devs.is_empty() {
        println!("Technology adoption comparison:");
        compare_tech_adoption(
            "Young Devs (18-24)",
            &young_devs,
            "Older Devs (45-54)",
            &older_devs,
        );
    }

    println!("✨ Demographic comparison complete!");
    Ok(())
}

fn analyze_group_demographics(group_name: &str, group: &[&indexmap::IndexMap<String, String>]) {
    println!("  {} Age Distribution:", group_name);
    let mut age_counts = std::collections::HashMap::new();

    for person in group {
        if let Some(age) = person.get("Age") {
            if !age.trim().is_empty() && age != "NA" {
                *age_counts.entry(age.clone()).or_insert(0) += 1;
            }
        }
    }

    for (age, count) in &age_counts {
        let percentage = (*count as f64 / group.len() as f64) * 100.0;
        println!("    {}: {} ({:.1}%)", age, count, percentage);
    }
    println!();
}

fn analyze_group_education(group_name: &str, group: &[&indexmap::IndexMap<String, String>]) {
    println!("  {} Education Levels:", group_name);
    let mut edu_counts = std::collections::HashMap::new();

    for person in group {
        if let Some(education) = person.get("EdLevel") {
            if !education.trim().is_empty() && education != "NA" {
                *edu_counts.entry(education.clone()).or_insert(0) += 1;
            }
        }
    }

    for (education, count) in &edu_counts {
        let percentage = (*count as f64 / group.len() as f64) * 100.0;
        println!("    {}: {} ({:.1}%)", education, count, percentage);
    }
    println!();
}

fn analyze_group_experience(group_name: &str, group: &[&indexmap::IndexMap<String, String>]) {
    println!("  {} Coding Experience:", group_name);
    let mut exp_counts = std::collections::HashMap::new();

    for person in group {
        if let Some(experience) = person.get("YearsCode") {
            if !experience.trim().is_empty() && experience != "NA" {
                *exp_counts.entry(experience.clone()).or_insert(0) += 1;
            }
        }
    }

    for (experience, count) in &exp_counts {
        let percentage = (*count as f64 / group.len() as f64) * 100.0;
        println!("    {}: {} ({:.1}%)", experience, count, percentage);
    }
    println!();
}

fn analyze_group_languages(group_name: &str, group: &[&indexmap::IndexMap<String, String>]) {
    println!("  {} Top Languages:", group_name);
    let mut lang_counts = std::collections::HashMap::new();

    for person in group {
        if let Some(languages) = person.get("LanguageHaveWorkedWith") {
            if !languages.trim().is_empty() && languages != "NA" {
                for lang in languages.split(';') {
                    let lang = lang.trim();
                    if !lang.is_empty() {
                        *lang_counts.entry(lang.to_string()).or_insert(0) += 1;
                    }
                }
            }
        }
    }

    // Sort by popularity
    let mut sorted_langs: Vec<_> = lang_counts.iter().collect();
    sorted_langs.sort_by(|a, b| b.1.cmp(a.1));

    for (lang, count) in sorted_langs.iter().take(5) {
        let percentage = (**count as f64 / group.len() as f64) * 100.0;
        println!("    {}: {} ({:.1}%)", lang, count, percentage);
    }
    println!();
}

fn analyze_so_usage(group_name: &str, group: &[&indexmap::IndexMap<String, String>]) {
    println!("  {} Stack Overflow Usage:", group_name);
    let mut so_freq_counts = std::collections::HashMap::new();

    for person in group {
        if let Some(so_freq) = person.get("SOVisitFreq") {
            if !so_freq.trim().is_empty() && so_freq != "NA" {
                *so_freq_counts.entry(so_freq.clone()).or_insert(0) += 1;
            }
        }
    }

    for (freq, count) in &so_freq_counts {
        let percentage = (*count as f64 / group.len() as f64) * 100.0;
        println!("    {}: {} ({:.1}%)", freq, count, percentage);
    }
    println!();
}

fn compare_tech_adoption(
    group1_name: &str,
    group1: &[&indexmap::IndexMap<String, String>],
    group2_name: &str,
    group2: &[&indexmap::IndexMap<String, String>],
) {
    let technologies = vec!["Python", "JavaScript", "Rust", "Java", "Go"];

    for tech in technologies {
        let group1_users = group1
            .iter()
            .filter(|person| {
                person
                    .get("LanguageHaveWorkedWith")
                    .map(|langs| langs.contains(tech))
                    .unwrap_or(false)
            })
            .count();

        let group2_users = group2
            .iter()
            .filter(|person| {
                person
                    .get("LanguageHaveWorkedWith")
                    .map(|langs| langs.contains(tech))
                    .unwrap_or(false)
            })
            .count();

        let group1_pct = (group1_users as f64 / group1.len() as f64) * 100.0;
        let group2_pct = (group2_users as f64 / group2.len() as f64) * 100.0;

        println!(
            "  {}: {} ({:.1}%) vs {} ({:.1}%)",
            tech, group1_name, group1_pct, group2_name, group2_pct
        );
    }
}
