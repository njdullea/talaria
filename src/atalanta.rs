use std::collections::HashMap;
use std::error::Error;

struct Atalanta {
	exchange_prices: HashMap<String, f64>,
}

impl Atalanta {
	pub fn new() -> Self {
		Self {
			exchange_prices: HashMap::new()
		}
	}

	pub fn update_exchange_price(&mut self, exchange: String, price: f64) {
		self.exchange_prices.insert(exchange, price);
	}

	pub fn find_best_trade_pair(&mut self) -> Option<((String, f64), (String, f64))> {
		let mut max_exchange: Option<(String, f64)> = None;
		let mut min_exchange: Option<(String, f64)> = None;

		self.exchange_prices.iter().for_each(|(exchange, price)| {
			let value = price.to_owned();

			// If there is no max exchange name, none of the values have been set yet.
			if max_exchange.is_none() {
				max_exchange = Some((exchange.to_owned(), value));
				min_exchange = Some((exchange.to_owned(), value));
			}
			
			if let Some((e, v)) = &max_exchange.clone() {
				if value > v.to_owned() {
					max_exchange = Some((e.to_owned(), v.to_owned()));
				}
			}

			if let Some((e, v)) = &min_exchange {
				if value < v.to_owned() {
					min_exchange = Some((e.to_owned(), v.to_owned()));
				}
			}
		});

		if max_exchange.is_some() && min_exchange.is_some() {
			let max_exchange_info = max_exchange.unwrap();
			let min_exchange_info = min_exchange.unwrap();
			// Confirm they are not the same exchange.
			if max_exchange_info.0 != min_exchange_info.0 {
				return Some((max_exchange_info, min_exchange_info));
			}
		}

		None
	}
}
