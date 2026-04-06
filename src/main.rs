mod tui_handler;
mod definition_handler;
mod conversion_handler;
mod eval;
mod eval_context;
mod history_io;
mod parser;
mod error;
mod constant;

// Variable name separator for numbered variables
// e.g., with VARIABLE_SEPARATOR = '_':
//   x_1, x_2, f_10, lin_5
// Change this to use a different separator (e.g., '-' for x-1, x-2)
pub const VARIABLE_SEPARATOR: char = '_';

// Helper function to escape the separator for regex patterns
pub fn escape_separator() -> String {
    // Escape regex metacharacters: . ^ $ * + ? ( ) [ ] { } | \
    match VARIABLE_SEPARATOR {
        '.' | '^' | '$' | '*' | '+' | '?' | '(' | ')' | '[' | ']' | '{' | '}' | '|' | '\\' => {
            format!("\\{}", VARIABLE_SEPARATOR)
        }
        _ => VARIABLE_SEPARATOR.to_string()
    }
}

use eval_context::EvalContext;
use eval::evaluate_input;
use history_io::import_history;
use tui_handler::tui_handler::TuiHandler;
use clap::Parser;
use color_eyre::Result;
use std::io::{self, Write};




//TODO: Add unit conversion and functions
//Add bracket highlighting and implicit brackets at ends



/// CLI/TUI Program for quick calculations on your terminal
#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// Enable tui
    #[arg(short, long)]
    tui: bool,
    /// Evaluate expression directly (e.g., "2 + 2")
    #[arg(short, long)]
    eval: Option<String>,
    /// Import a session file at startup (e.g., --import session.json)
    #[arg(long)]
    import: Option<String>,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    // Direct eval mode: calcli -e "2 + 2"
    if let Some(expr) = args.eval {
        let mut eval_ctx = EvalContext::new();
        match evaluate_input(&mut eval_ctx, &expr) {
            Ok(result) => {
                println!("{}", eval_ctx.format_result(result));
                return Ok(());
            }
            Err(e) => {
                eprintln!("{}", e);
                return Ok(());
            }
        }
    }

    // TUI
    if args.tui {
        let mut terminal = ratatui::init();
        let mut app = TuiHandler::new();
        if let Some(path) = &args.import {
            app.preload_history(path);
        }
        let result = app.run(&mut terminal);
        ratatui::restore();
        return result;
    }

    // CLI (default)
    println!("Calculator CLI - Enter expressions to evaluate (type 'quit' or 'q' to exit)");
    let mut eval_ctx = EvalContext::new();

    if let Some(path) = &args.import {
        match import_history(path) {
            Ok(entries) => {
                for entry in entries {
                    match evaluate_input(&mut eval_ctx, &entry.expression) {
                        Ok(result) => {
                            println!("{}) {} = {}", eval_ctx.counter, entry.expression.trim(), result);
                            eval_ctx.history_entries.push((entry.expression, result));
                            eval_ctx.counter += 1;
                        }
                        Err(e) => eprintln!("Import error on '{}': {}", entry.expression, e),
                    }
                }
            }
            Err(e) => eprintln!("Failed to import '{}': {}", path, e),
        }
    }

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if (input == "quit") || (input == "q") {
            break Ok(());
        }

        if let Some(path) = input.strip_prefix(":w ").or_else(|| input.strip_prefix(":export ")) {
            match history_io::export_history(path.trim(), &eval_ctx.history_entries) {
                Ok(()) => println!("Exported history to {}", path.trim()),
                Err(e) => eprintln!("Export error: {}", e),
            }
            continue;
        }

        if let Some(path) = input.strip_prefix(":r ").or_else(|| input.strip_prefix(":import ")) {
            match import_history(path.trim()) {
                Ok(entries) => {
                    for entry in entries {
                        match evaluate_input(&mut eval_ctx, &entry.expression) {
                            Ok(result) => {
                                println!("{}) {} = {}", eval_ctx.counter, entry.expression.trim(), result);
                                eval_ctx.history_entries.push((entry.expression, result));
                                eval_ctx.counter += 1;
                            }
                            Err(e) => eprintln!("Import error on '{}': {}", entry.expression, e),
                        }
                    }
                    println!("Imported history from {}", path.trim());
                }
                Err(e) => eprintln!("Import error: {}", e),
            }
            continue;
        }

        match evaluate_input(&mut eval_ctx, &input) {
            Ok(result) => {
                eval_ctx.history_entries.push((input.to_string(), result));
                println!("{}) = {}", eval_ctx.counter, eval_ctx.format_result(result));
                eval_ctx.counter += 1;
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
}


