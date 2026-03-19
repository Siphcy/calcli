mod input_handler;
mod eval;


use eval::evaluate_input;
use meval::{eval_str, Context};
use input_handler::InputHandler;
use clap::Parser;
use color_eyre::Result;
use std::io::{self, Write};


//TODO: Add unit conversion and shit

/// CLI/TUI Program for quick calculations on your terminal
#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// Enable tui
    #[arg(short, long)]
    tui: bool,
    /// Evaluate expression directly (e.g., "2 + 2")
    #[arg(short, long)]
    eval: Option<String>
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

    // TUI mode: calcli -t
    if args.tui {
        let mut terminal = ratatui::init();
        let app = InputHandler::new();
        let result = app.run(&mut terminal);
        ratatui::restore();
        return result;
    }

    // Default: CLI REPL mode
    println!("Calculator CLI - Enter expressions to evaluate (type 'quit' to exit)");
    let mut parse_result: Vec<f64> = Vec::new();
    let mut counter: usize = 1;
    let mut ctx = Context::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "quit" {
            break Ok(());
        }

        if let Some(result) = evaluate_input(counter, input, &mut parse_result, &mut ctx) {
            println!("{}) = {}", counter, result);
            counter += 1;
        }
    }
}


