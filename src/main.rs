mod talaria;
mod local_env;
mod exchange;
mod record;
mod traits;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match parse_config(&args).to_owned() {
        Ok(arg) => {
            if arg == "reset-data" {
                match exchange::save_exchange_data() {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Error saving exchange data: {:?}", e);
                    }
                }
            } else if arg == "backtest" {
                // let mut ata = 
                let mut tlr = talaria::Talaria::new();
                tlr.backtest();
            } else {
                println!("Not a valid action!");
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
