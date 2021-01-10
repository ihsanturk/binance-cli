extern crate binance;
extern crate chrono;
#[macro_use]
extern crate clap;

use binance::api::*;
use chrono::{Utc, TimeZone, Local};
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
				.required(true))
		)

		.subcommand(App::new("trades")
			.setting(AppSettings::ColoredHelp)
			.about("Prints all the trades made in the given parity")
			.arg(Arg::new("SYMBOL")
				.about("Symbol, example: btcusdt")
				.index(1)
				.required(true))
			.arg(Arg::new("FORMAT")
				.about("Output format, available values: { ledger }")
				.short('o')
				.long("output-as")
				.required(false)
				.takes_value(true))
		);

	let matches = app.clone().get_matches();

	let account: Account = Binance::new(
		get_env_var("BINANCE_API_KEY"),
		get_env_var("BINANCE_SEC_KEY")
	);

	// balance
	if let Some(ref matches) = matches.subcommand_matches("balance") {
		let asset = matches.value_of("ASSET");
		match account.get_balance(asset.unwrap().to_uppercase()) {
			Ok(response) => println!("{:?}", response),
			Err(e) => println!("Error: {}", e),
		};
	}

	// trades
	else if let Some(ref matches) = matches.subcommand_matches("trades") {
		let symbol = matches.value_of("SYMBOL");
		match account.trade_history(symbol.unwrap().to_uppercase()) {
			Ok(trades) => {
				for trade in trades {
					println!("{}", format(trade, symbol.unwrap().to_string()));
				}
			},
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


fn format(trade: binance::model::TradeHistory, symbol: String) -> String {

	let indent = "    ";

	let date = Utc.timestamp_millis(trade.time as i64)
			.with_timezone(&Local)
			.format("%Y-%m-%d %X")
			.to_string();

	let side: Option<&str> = match trade.is_buyer {
		true => Some("buy"),
		false => Some("sell"),
	};

	// FIXME: does not work for 4 char left_assets: "LINKUSDT" or "LINKBTC"
	//        SOLUTION: take symbol from user with a separate format:
	//                  "link-usdt" or "link/usdt" and split from that char
	let (left_asset, right_asset) = symbol.split_at(3);
	let left_amount = trade.qty;
	let right_amount = trade.price * trade.qty;

	let left = format!("{} {}", left_amount, left_asset);
	let right = format!("{} {}", right_amount, right_asset);
	let p1account = match side.unwrap() {
		"buy" => Some(left_asset), "sell" => Some(right_asset), _ => None,
	};
	let p2account = match side.unwrap() {
		"buy" => Some(right_asset), "sell" => Some(left_asset), _ => None,
	};

	let header_trade = format!("{date} * {description}",
		date = date,
		description = format!("{} {}", side.unwrap(), left_asset),
	);
	let amounts =  match side.unwrap() {
		"buy" => Some(format!("{} @@ {}", left, right)),
		"sell" => Some(format!("{} @@ {}", right, left)),
		_ => None,
	};
	let posting1_trade = format!(
		"asset:binance.com:{account}{indent}{amounts}",
		indent = indent,
		account = p1account.unwrap(),
		amounts = amounts.unwrap(),
	);
	let posting2_trade = format!(
		"asset:binance.com:{account}",
		account = p2account.unwrap(),
	);
	let transaction_trade = format!("{header}\n{indent}{p1}\n{indent}{p2}\n",
		header = header_trade,
		indent = indent,
		p1 = posting1_trade,
		p2 = posting2_trade,
	);

	let transaction_fee = format!("{header}\n{indent}{p1}\n{indent}{p2}\n",
		header = format!("{} fee", header_trade),
		indent = indent,
		p1 = format!("asset:binance.com:fee{indent}{amount} {asset}",
			indent = indent,
			amount = trade.commission,
			asset = trade.commission_asset.to_lowercase(),
		),
		p2 = format!("asset:binance.com:{}",
			trade.commission_asset.to_lowercase()),
	);

	let transactions = format!("{t1}\n{t2}",
		t1 = transaction_trade,
		t2 = transaction_fee
	);

	return transactions;

}
