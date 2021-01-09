extern crate binance;

use binance::api::*;
use binance::account::*;

fn main() {

	let api_key_env = "BINANCE_API_KEY";
	let api_key: Option<String> = match std::env::var(api_key_env) {
		Ok(value) => Some(value),
		Err(e) => {
			eprintln!("{}: {:?}", e, api_key_env);
			None
		},
	};

	let sec_key_env = "BINANCE_SEC_KEY";
	let secret_key: Option<String> = match std::env::var(sec_key_env) {
		Ok(value) => Some(value),
		Err(e) => {
			eprintln!("{}: {:?}", e, sec_key_env);
			None
		},
	};

	let account: Account = Binance::new(api_key, secret_key);

	match account.trade_history("ETHBTC") {
		Ok(answer) => println!("{:?}", answer),
		Err(e) => println!("Error: {}", e),
	};

}
