extern crate binance;
#[macro_use]
extern crate clap;

use binance::api::*;
use binance::account::*;
use clap::{Arg, App, AppSettings};

fn main() {

	let matches = App::new(crate_name!())
		.setting(AppSettings::ColoredHelp)
		.version(crate_version!())
		.author(crate_authors!())
		.about(crate_description!())

		.subcommand(App::new("balance")
			.setting(AppSettings::ColoredHelp)
			.arg(Arg::new("asset")
				.short('a')
				.long("asset")
				.required(true)
				.about("Asset, ex: btc")
				.takes_value(true)))

		.subcommand(App::new("trades")
			.setting(AppSettings::ColoredHelp)
			.arg(Arg::new("symbol")
				.short('s')
				.long("symbol")
				.required(true)
				.about("Symbol, ex: btcusdt")
				.takes_value(true))
			.arg(Arg::new("output")
				.short('o')
				.long("output")
				.required(false)
				.about("Output format, ex: ledger")
				.takes_value(true)))

		.get_matches();

	let account: Account = Binance::new(
		get_env_var("BINANCE_API_KEY"),
		get_env_var("BINANCE_SEC_KEY")
	);

	// balance
	if let Some(ref matches) = matches.subcommand_matches("balance") {
		let asset = matches.value_of("asset");
		match account.get_balance(asset.unwrap().to_uppercase()) {
			Ok(response) => println!("{:?}", response), // format_output(response, "ledger")
			Err(e) => println!("Error: {}", e),
		};
	}

	// trades
	else if let Some(ref matches) = matches.subcommand_matches("trades") {
		let symbol = matches.value_of("symbol");
		match account.trade_history(symbol.unwrap().to_uppercase()) {
			Ok(response) => println!("{:?}", response),
			Err(e) => println!("Error: {}", e),
		};
	}

}

fn get_env_var(key: &str) -> Option<String> {
	match std::env::var(key) {
		Ok(value) => Some(value),
		Err(e) => {
			eprintln!("{}: {:?}", e, key);
			std::process::exit(2);
		},
	}
}

// fn output_as_ledger_format(trades) {
// }
