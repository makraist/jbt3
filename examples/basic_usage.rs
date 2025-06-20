use so_survey_analyzer::{SurveyAnalyzer, SurveyError};

fn main() -> Result<(), SurveyError> {
    println!("Stack Overflow Survey Analyzer - Basic Usage Example");
    println!("====================================================");

    // Load the survey data
    println!("Loading survey data...");
    let analyzer = SurveyAnalyzer::from_excel("../so_2024_raw.xlsx")?;
    
    let questions = analyzer.get_survey_structure();
    println!("✓ Loaded {} questions with {} respondents\n", 
             questions.len(), 
             analyzer.survey().respondent_count());

    // Example 1: Search for questions about programming languages
    println!("1. Searching for language-related questions:");
    let lang_questions = analyzer.search_questions("language");
    for question in lang_questions.iter().take(3) {
        println!("   • Question {}: {}", question.id, question.text);
    }
    println!();

    // Example 2: Analyze remote work distribution
    println!("2. Analyzing remote work patterns:");
    let remote_questions = analyzer.search_questions("remote");
    if let Some(question) = remote_questions.first() {
        match analyzer.get_distribution(question.id) {
            Ok(distribution) => {
                println!("   Question: {}", question.text);
                if let Some((most_popular, count, percentage)) = distribution.most_popular() {
                    println!("   Most popular: {} ({} responses, {:.1}%)", 
                             most_popular, count, percentage);
                }
                
                let above_20_percent = distribution.above_threshold(20.0);
                println!("   Options with >20% share:");
                for (option, count, percentage) in above_20_percent {
                    println!("     - {}: {} ({:.1}%)", option, count, percentage);
                }
            }
            Err(e) => println!("   Error analyzing distribution: {}", e),
        }
    }
    println!();

    // Example 3: Create and analyze subsets
    println!("3. Creating respondent subsets:");
    if let Some(question) = remote_questions.first() {
        match analyzer.create_subset(question.id, "Remote") {
            Ok(remote_workers) => {
                println!("   Remote workers: {} respondents ({:.1}% of total)", 
                         remote_workers.size(), remote_workers.percentage());
                
                // Try to find another subset to intersect with
                let employment_questions = analyzer.search_questions("employment");
                if let Some(emp_question) = employment_questions.first() {
                    match analyzer.create_subset(emp_question.id, "full-time") {
                        Ok(fulltime_workers) => {
                            let intersection = remote_workers.intersect(&fulltime_workers);
                            println!("   Remote full-time workers: {} respondents", 
                                     intersection.len());
                        }
                        Err(_) => println!("   Could not create full-time subset"),
                    }
                }
            }
            Err(e) => println!("   Error creating subset: {}", e),
        }
    }
    println!();

    // Example 4: Show survey structure overview
    println!("4. Survey structure overview:");
    println!("   First 5 questions:");
    for question in questions.iter().take(5) {
        println!("   • Q{}: {} (Type: {:?})", 
                 question.id, 
                 question.text.chars().take(50).collect::<String>() + "...",
                 question.question_type);
    }
    
    println!("\n✓ Analysis complete!");
    Ok(())
}