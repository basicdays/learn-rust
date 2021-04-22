use std::{env, error::Error};

use ch_12_minigrep as minigrep;
use minigrep::Config;

fn main() -> Result<(), Box<dyn Error>> {
	let config = Config::new(env::args())?;

	minigrep::run(config)
}
