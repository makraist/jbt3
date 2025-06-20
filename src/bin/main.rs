use clap::{Parser, Subcommand};
use stack_overflow_survey_analyzer::SurveyData;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "so_analyzer")]
#[command(about = "A CLI tool for analyzing Stack Overflow Survey data")]
#[command(version = "0.1.0")]
struct Cli {
    /// Path to the Excel file containing survey data
    #[arg(short, long, default_value = "so_2024_raw2.xlsx")]
    file: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Display the survey structure (list of questions)
    Structure,
    /// Search for questions by keyword
    Search {
        /// Keyword to search for in question text or column name
        keyword: String,
    },
    /// Display distribution of answers for a question
    Distribution {
        /// Column name of the question
        column: String,
    },
    /// Create subset of respondents based on question and option
    Subset {
        /// Column name of the question
        column: String,
        /// Option value to filter by
        option: String,
        /// Show detailed responses instead of just count
        #[arg(short, long)]
        detailed: bool,
    },
    /// Get all available options for a question
    Options {
        /// Column name of the question
        column: String,
    },
    /// Interactive mode - start a REPL-like interface
    Interactive,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Load survey data
    println!("Loading survey data from: {}", cli.file.display());
    let data = SurveyData::load_from_excel(&cli.file)?;
    println!("Data loaded successfully!\n");

    match cli.command {
        Commands::Structure => {
            data.display_survey_structure();
        }
        Commands::Search { keyword } => {
            let results = data.search_questions(&keyword);
            if results.is_empty() {
                println!("No questions found matching '{}'", keyword);
            } else {
                println!(
                    "Found {} question(s) matching '{}':\n",
                    results.len(),
                    keyword
                );
                for (i, question) in results.iter().enumerate() {
                    println!(
                        "{}. [{}] {} (Type: {:?})",
                        i + 1,
                        question.column,
                        question.question_text,
                        question.question_type
                    );
                }
            }
        }
        Commands::Distribution { column } => {
            data.display_distribution(&column)?;
        }
        Commands::Subset {
            column,
            option,
            detailed,
        } => {
            let subset = data.create_subset(&column, &option)?;
            println!(
                "Found {} respondents who selected '{}' for '{}'",
                subset.len(),
                option,
                column
            );

            if detailed && !subset.is_empty() {
                println!("\nDetailed responses:");
                for (i, response) in subset.iter().take(10).enumerate() {
                    println!("\n--- Response {} ---", i + 1);
                    for (key, value) in response.iter() {
                        if !value.trim().is_empty() && value != "NA" {
                            println!("{}: {}", key, value);
                        }
                    }
                }
                if subset.len() > 10 {
                    println!("\n... and {} more responses", subset.len() - 10);
                }
            }
        }
        Commands::Options { column } => {
            let options = data.get_question_options(&column)?;
            if let Some(question) = data.questions.get(&column) {
                println!(
                    "Available options for '{}' (Type: {:?}):",
                    question.question_text, question.question_type
                );
                println!("Total options: {}\n", options.len());
                for (i, option) in options.iter().enumerate() {
                    println!("{}. {}", i + 1, option);
                }
            }
        }
        Commands::Interactive => {
            run_interactive_mode(data)?;
        }
    }

    Ok(())
}

fn run_interactive_mode(data: SurveyData) -> anyhow::Result<()> {
    use std::io::{self, Write};

    println!("=== Interactive Stack Overflow Survey Analyzer ===");
    println!("Type 'help' for available commands, 'quit' to exit\n");

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        let command = parts[0].to_lowercase();

        match command.as_str() {
            "help" => {
                println!("Available commands:");
                println!("  structure                    - Display survey structure");
                println!("  search <keyword>             - Search for questions");
                println!("  dist <column>                - Show distribution for a question");
                println!("  subset <column> <option>     - Create subset of respondents");
                println!("  options <column>             - Show available options for a question");
                println!("  columns                      - List all column names");
                println!("  quit                         - Exit interactive mode");
                println!();
            }
            "quit" | "exit" => {
                println!("Goodbye!");
                break;
            }
            "structure" => {
                data.display_survey_structure();
            }
            "columns" => {
                println!("Available columns:");
                for (i, column) in data.questions.keys().enumerate() {
                    println!("{}. {}", i + 1, column);
                }
                println!();
            }
            "search" => {
                if parts.len() < 2 {
                    println!("Usage: search <keyword>");
                    continue;
                }
                let keyword = parts[1..].join(" ");
                let results = data.search_questions(&keyword);
                if results.is_empty() {
                    println!("No questions found matching '{}'", keyword);
                } else {
                    println!(
                        "Found {} question(s) matching '{}':\n",
                        results.len(),
                        keyword
                    );
                    for (i, question) in results.iter().enumerate() {
                        println!(
                            "{}. [{}] {} (Type: {:?})",
                            i + 1,
                            question.column,
                            question.question_text,
                            question.question_type
                        );
                    }
                }
                println!();
            }
            "dist" => {
                if parts.len() < 2 {
                    println!("Usage: dist <column>");
                    continue;
                }
                let column = parts[1];
                if let Err(e) = data.display_distribution(column) {
                    println!("Error: {}", e);
                }
                println!();
            }
            "subset" => {
                if parts.len() < 3 {
                    println!("Usage: subset <column> <option>");
                    continue;
                }
                let column = parts[1];
                let option = parts[2..].join(" ");
                match data.create_subset(column, &option) {
                    Ok(subset) => {
                        println!(
                            "Found {} respondents who selected '{}' for '{}'",
                            subset.len(),
                            option,
                            column
                        );
                    }
                    Err(e) => println!("Error: {}", e),
                }
                println!();
            }
            "options" => {
                if parts.len() < 2 {
                    println!("Usage: options <column>");
                    continue;
                }
                let column = parts[1];
                match data.get_question_options(column) {
                    Ok(options) => {
                        if let Some(question) = data.questions.get(column) {
                            println!(
                                "Available options for '{}' (Type: {:?}):",
                                question.question_text, question.question_type
                            );
                            println!("Total options: {}\n", options.len());
                            for (i, option) in options.iter().take(20).enumerate() {
                                println!("{}. {}", i + 1, option);
                            }
                            if options.len() > 20 {
                                println!("... and {} more options", options.len() - 20);
                            }
                        }
                    }
                    Err(e) => println!("Error: {}", e),
                }
                println!();
            }
            _ => {
                println!(
                    "Unknown command: '{}'. Type 'help' for available commands.",
                    command
                );
            }
        }
    }

    Ok(())
}
