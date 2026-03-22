mod input_handler;
mod eval;
mod unit_conversion;
mod eval_context;
mod vi_inputs;

use eval_context::EvalContext;
use eval::evaluate_input;
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

    // TUI
    if args.tui {
        let mut terminal = ratatui::init();
        let app = InputHandler::new();
        let result = app.run(&mut terminal);
        ratatui::restore();
        return result;
    }

    // CLI (default)
    println!("Calculator CLI - Enter expressions to evaluate (type 'quit' or 'q' to exit)");
    let mut eval_ctx = EvalContext::new();

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

        match evaluate_input(&mut eval_ctx, &input) {
            Ok(result) => {
                println!("{}) = {}", eval_ctx.counter, result);
                eval_ctx.counter += 1;
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
}


