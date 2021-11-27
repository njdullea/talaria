mod talaria;
mod local_env;
mod exchange;
mod record;
mod traits;
mod exchanges;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match parse_config(&args).to_owned() {
        Ok(arg) => {
            if arg == "reset-data" {
                reset_data();
            } else if arg == "backtest" {
                execute_backtest();
            } else if arg == "talaria" {
                execute_talaria();
            }
        },
        Err(err) => {
            println!("Error in parse config: {:?}", err);
        },
    }
}

fn parse_config(args: &[String]) -> Result<&str, &'static str> {
    match args.get(1) {
        None => Err("Please provide and execution argument."),
        Some(e) => Ok(e)
    }
}

fn execute_talaria() {
    // TODO: when we execute for real, we need to make sure that if either of the websockets fails
    // that way immediately stop everything else.
    println!("Would execute talaria now!");
}

fn execute_backtest() {
    let mut tlr = talaria::Talaria::new();
    tlr.backtest();
}

fn reset_data() {
    match exchange::save_exchange_data() {
        Ok(_) => {}
        Err(e) => {
            println!("Error saving exchange data: {:?}", e);
        }
    }
}
