mod talaria;
mod local_env;
mod market;
mod record;
mod traits;

fn main() {
    match market::save_exchange_data() {
        Ok(_) => {}
        Err(e) => {
            println!("Error saving exchange data: {:?}", e);
        }
    }
}
