mod talaria;
mod local_env;
mod exchange;
mod record;
mod traits;

fn main() {
    match exchange::save_exchange_data() {
        Ok(_) => {}
        Err(e) => {
            println!("Error saving exchange data: {:?}", e);
        }
    }
}
