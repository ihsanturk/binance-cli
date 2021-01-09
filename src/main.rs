extern crate binance;
#[macro_use]
extern crate clap;

use binance::api::*;
use binance::account::*;
use clap::{Arg, App, AppSettings};

fn main() {

	let mut app = App::new("binance")
		.setting(AppSettings::ColoredHelp)
		.version(crate_version!())
		.author(crate_authors!())
		.about(crate_description!())

		.subcommand(App::new("balance")
			.setting(AppSettings::ColoredHelp)
			.about("Prints balance of the given asset in your account")
			.arg(Arg::new("ASSET")
				.about("Asset, example: btc")
				.index(1)
				.required(true)))

		.subcommand(App::new("trades")
			.setting(AppSettings::ColoredHelp)
			.about("Prints all the trades made in the given parity")
			.arg(Arg::new("SYMBOL")
				.about("Symbol, example: btcusdt")
				.index(1)
				.required(true))
			.arg(Arg::new("FORMAT")
				.short('o')
				.long("output-as")
				.required(false)
				.about("Output format, available values: { ledger }")
				.takes_value(true)));

	let matches = app.clone().get_matches();

	let account: Account = Binance::new(
		get_env_var("BINANCE_API_KEY"),
		get_env_var("BINANCE_SEC_KEY")
	);

	// balance
	if let Some(ref matches) = matches.subcommand_matches("balance") {
		let asset = matches.value_of("ASSET");
		match account.get_balance(asset.unwrap().to_uppercase()) {
			Ok(response) => println!("{:?}", response), // format_output(response, "ledger")
			Err(e) => println!("Error: {}", e),
		};
	}

	// trades
	else if let Some(ref matches) = matches.subcommand_matches("trades") {
		let symbol = matches.value_of("SYMBOL");
		match account.trade_history(symbol.unwrap().to_uppercase()) {
			Ok(response) => println!("{:?}", response),
			Err(e) => println!("Error: {}", e),
		};
	}

	// {no sub command}
	else {
		app.print_help().unwrap();
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
