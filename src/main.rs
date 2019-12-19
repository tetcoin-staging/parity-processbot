use rocksdb::DB;
use std::collections::HashMap;
use std::fs::File;
use std::time::Duration;

use parity_processbot::bots;
use parity_processbot::github_bot;
use parity_processbot::matrix_bot;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	match run().await {
		Err(error) => panic!("{}", error),
		_ => Ok(()),
	}
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
	env_logger::from_env(env_logger::Env::default().default_filter_or("info")).init();

	dotenv::dotenv().ok();
	let db_path = dotenv::var("DB_PATH").expect("DB_PATH");
	let github_organization = dotenv::var("GITHUB_ORGANIZATION").expect("GITHUB_ORGANIZATION");
	let github_token = dotenv::var("GITHUB_TOKEN").expect("GITHUB_TOKEN");
	let matrix_homeserver = dotenv::var("MATRIX_HOMESERVER").expect("MATRIX_HOMESERVER");
	let matrix_user = dotenv::var("MATRIX_USER").expect("MATRIX_USER");
	let matrix_password = dotenv::var("MATRIX_PASSWORD").expect("MATRIX_PASSWORD");
	let matrix_channel_id = dotenv::var("MATRIX_CHANNEL_ID").expect("MATRIX_CHANNEL_ID");
	let engineers_path = dotenv::var("ENGINEERS_PATH").expect("ENGINEERS_PATH");
	let tick_secs = dotenv::var("TICK_SECS")
		.expect("TICK_SECS")
		.parse::<u64>()
		.expect("parse tick_secs");

	/*
	let mut engineers: HashMap<String, github_bot::Engineer> = HashMap::new();
	let mut rdr =
		csv::Reader::from_reader(File::open(engineers_path).expect("open engineers file"));
	for result in rdr.deserialize() {
		let record: github_bot::Engineer = result?;
		if let Some(ref github) = record.github {
			engineers.insert(github.clone(), record);
		}
	}
	*/

	let db = DB::open_default(db_path)?;

	let matrix_bot =
		matrix_bot::MatrixBot::new(&matrix_homeserver, &matrix_user, &matrix_password)?;

	let github_bot = github_bot::GithubBot::new(&github_organization, &github_token)?;

	println!("[+] Connected to {} as {}", matrix_homeserver, matrix_user);

	let mut interval = tokio::time::interval(Duration::from_secs(tick_secs));
	loop {
		interval.tick().await;
		bots::update(&db, &github_bot, &matrix_bot)?;
	}
}
