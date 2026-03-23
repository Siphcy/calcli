mod input_handler;
mod eval;
mod unit_conversion;
mod eval_context;
mod vi_inputs;
mod history_io;

use eval_context::EvalContext;
use eval::evaluate_input;
use history_io::import_history;
use meval::eval_str;
use input_handler::InputHandler;
use clap::Parser;
use color_eyre::Result;
use std::io::{self, Write};



//TODO: Add unit conversion and functions



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
        let result = eval_str(&expr)?;
        println!("{}", result);
        return Ok(());
    }

    // TUI
    if args.tui {
        let mut terminal = ratatui::init();
        let mut app = InputHandler::new();
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
        //TODO: using unwrap like this can be dangerous but idk maybe i'l cehck later
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
                println!("{}) = {}", eval_ctx.counter, result);
                eval_ctx.counter += 1;
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
}


