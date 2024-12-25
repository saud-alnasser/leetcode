use clap::{Parser, Subcommand};
use std::{fs, path::Path};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    New {
        id: u32,
        #[arg(value_parser = Language::parse)]
        language: Language,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum Language {
    Rust,
}

impl Language {
    pub fn parse(value: &str) -> Result<Self, String> {
        match value {
            "rust" | "rs" => Ok(Self::Rust),
            _ => Err("unsupported language".to_string()),
        }
    }

    pub fn ext(&self) -> &str {
        match self {
            Self::Rust => "rs",
        }
    }
}

static README_TEMPLATE: &str = include_str!("../assets/README.md");

static RS_LIB_TEMPLATE: &str = include_str!("../assets/[rs]/src/lib.rs");
static RS_SOLUTION_TEMPLATE: &str = include_str!("../assets/[rs]/src/solution.rs");
static RS_CARGO_PROBLEM_TEMPLATE: &str = include_str!("../assets/[rs]/Cargo.toml");

impl Command {
    pub fn execute(&self) {
        match self {
            Command::New { id, language } => Command::new(*id, *language),
        }
    }

    fn new(id: u32, language: Language) {
        // problems directory
        let problem_dir = Path::new("./problems");
        if !problem_dir.exists() {
            fs::create_dir(problem_dir).expect("failed to create problems directory");
        }

        // problem directory
        let problem_path = problem_dir.join(format!("{} ({})", id, language.ext()));
        if !problem_path.exists() {
            fs::create_dir(&problem_path).expect("failed to create problem directory");
        } else {
            panic!("problem already exists");
        }

        // cargo.toml
        let problem_file = problem_path.join("Cargo.toml");
        let problem_content = RS_CARGO_PROBLEM_TEMPLATE.replace("[id]", &id.to_string());
        fs::write(problem_file, problem_content).expect("failed to write problem file");

        // readme
        let readme_file = problem_path.join("README.md");
        fs::write(readme_file, README_TEMPLATE).expect("failed to write problem file");

        // src
        let src_path = problem_path.join("src");
        fs::create_dir(&src_path).expect("failed to create src directory");

        // lib.rs
        let lib_file = src_path.join("lib.rs");
        fs::write(lib_file, RS_LIB_TEMPLATE).expect("failed to write lib file");

        // solution.rs
        let solution_file = src_path.join("solution.rs");
        fs::write(solution_file, RS_SOLUTION_TEMPLATE).expect("failed to write solution file");
    }
}

fn main() {
    let args = Args::parse();

    match args.command {
        Some(command) => command.execute(),
        None => {
            eprintln!("no valid command provided");
            std::process::exit(1);
        }
    }
}
