use clap::{Parser, Subcommand};
use so_survey_analyzer::{SurveyAnalyzer, SurveyError};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "so-survey-cli")]
#[command(about = "Stack Overflow Survey Data Analyzer")]
#[command(version = "0.1.0")]
struct Cli {
    /// Path to the Excel survey data file
    #[arg(short, long, default_value = "../so_2024_raw.xlsx")]
    file: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Display the survey structure (list of questions)
    Structure {
        /// Show only first N questions
        #[arg(short, long)]
        limit: Option<usize>,
        /// Show questions containing this term
        #[arg(short, long)]
        filter: Option<String>,
    },
    /// Search for questions or options
    Search {
        /// Search term
        term: String,
        /// Search in questions (default) or options
        #[arg(short, long)]
        options: bool,
    },
    /// Create a subset of respondents
    Subset {
        /// Question ID
        question_id: usize,
        /// Answer option to filter by
        option: String,
    },
    /// Display answer distribution for a question
    Distribution {
        /// Question ID
        question_id: usize,
        /// Minimum percentage threshold to display
        #[arg(short, long, default_value = "0.0")]
        threshold: f64,
    },
    /// Interactive REPL mode
    Repl,
}

fn main() -> Result<(), SurveyError> {
    let cli = Cli::parse();

    println!("Loading survey data from: {:?}", cli.file);
    let analyzer = SurveyAnalyzer::from_excel(&cli.file)?;
    println!("Loaded {} questions with {} total respondents\n", 
             analyzer.get_survey_structure().len(),
             analyzer.survey().respondent_count());

    match cli.command {
        Commands::Structure { limit, filter } => {
            let questions = analyzer.get_survey_structure();
            let mut filtered_questions: Vec<_> = questions.iter().collect();

            if let Some(filter_term) = filter {
                filtered_questions = analyzer.search_questions(&filter_term);
            }

            if let Some(limit) = limit {
                filtered_questions.truncate(limit);
            }

            println!("Survey Structure ({} questions):", filtered_questions.len());
            println!("{:-<80}", "");
            
            for question in filtered_questions {
                println!("Question {}: {}", question.id, question.text);
                println!("  Type: {:?}", question.question_type);
                if !question.options.is_empty() {
                    println!("  Options: {}", question.options.join(", "));
                }
                println!();
            }
        }

        Commands::Search { term, options } => {
            if options {
                let results = analyzer.search_options(&term);
                println!("Found {} option(s) containing '{}':", results.len(), term);
                for (question_id, option) in results {
                    println!("  Question {}: {}", question_id, option);
                }
            } else {
                let results = analyzer.search_questions(&term);
                println!("Found {} question(s) containing '{}':", results.len(), term);
                for question in results {
                    println!("  Question {}: {}", question.id, question.text);
                }
            }
        }

        Commands::Subset { question_id, option } => {
            let subset = analyzer.create_subset(question_id, &option)?;
            println!("{}", subset.display());
        }

        Commands::Distribution { question_id, threshold } => {
            let distribution = analyzer.get_distribution(question_id)?;
            println!("{}", distribution.display());
            
            if threshold > 0.0 {
                let above_threshold = distribution.above_threshold(threshold);
                if !above_threshold.is_empty() {
                    println!("\nAnswers above {:.1}% threshold:", threshold);
                    for (option, count, percentage) in above_threshold {
                        println!("  {}: {} ({:.1}%)", option, count, percentage);
                    }
                }
            }
        }

        Commands::Repl => {
            run_repl(analyzer)?;
        }
    }

    Ok(())
}

fn run_repl(analyzer: SurveyAnalyzer) -> Result<(), SurveyError> {
    println!("Welcome to the Stack Overflow Survey Analyzer REPL!");
    println!("Available commands:");
    println!("  list [limit] - List questions (optionally limit to N questions)");
    println!("  search <term> - Search questions");
    println!("  searchopt <term> - Search options");
    println!("  dist <question_id> - Show distribution for question");
    println!("  subset <question_id> <option> - Create subset");
    println!("  help - Show this help");
    println!("  quit - Exit");
    println!();

    loop {
        print!("survey> ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_err() {
            break;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "quit" | "exit" => break,
            "help" => {
                println!("Available commands:");
                println!("  list [limit] - List questions");
                println!("  search <term> - Search questions");
                println!("  searchopt <term> - Search options");
                println!("  dist <question_id> - Show distribution");
                println!("  subset <question_id> <option> - Create subset");
                println!("  help - Show this help");
                println!("  quit - Exit");
            }
            "list" => {
                let limit = parts.get(1).and_then(|s| s.parse().ok());
                let questions = analyzer.get_survey_structure();
                let count = limit.unwrap_or(questions.len()).min(questions.len());
                
                for question in questions.iter().take(count) {
                    println!("{}: {}", question.id, question.text);
                }
                println!("({} of {} questions shown)", count, questions.len());
            }
            "search" => {
                if parts.len() < 2 {
                    println!("Usage: search <term>");
                    continue;
                }
                let term = parts[1..].join(" ");
                let results = analyzer.search_questions(&term);
                for question in results {
                    println!("{}: {}", question.id, question.text);
                }
            }
            "searchopt" => {
                if parts.len() < 2 {
                    println!("Usage: searchopt <term>");
                    continue;
                }
                let term = parts[1..].join(" ");
                let results = analyzer.search_options(&term);
                for (question_id, option) in results {
                    println!("Q{}: {}", question_id, option);
                }
            }
            "dist" => {
                if parts.len() < 2 {
                    println!("Usage: dist <question_id>");
                    continue;
                }
                if let Ok(question_id) = parts[1].parse::<usize>() {
                    match analyzer.get_distribution(question_id) {
                        Ok(distribution) => println!("{}", distribution.display()),
                        Err(e) => println!("Error: {}", e),
                    }
                } else {
                    println!("Invalid question ID");
                }
            }
            "subset" => {
                if parts.len() < 3 {
                    println!("Usage: subset <question_id> <option>");
                    continue;
                }
                if let Ok(question_id) = parts[1].parse::<usize>() {
                    let option = parts[2..].join(" ");
                    match analyzer.create_subset(question_id, &option) {
                        Ok(subset) => println!("{}", subset.display()),
                        Err(e) => println!("Error: {}", e),
                    }
                } else {
                    println!("Invalid question ID");
                }
            }
            _ => println!("Unknown command: {}. Type 'help' for available commands.", parts[0]),
        }
        println!();
    }

    println!("Goodbye!");
    Ok(())
}