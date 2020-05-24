use std::{env, error::Error};

use ch_12_minigrep as minigrep;
use minigrep::Config;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args)?;

    minigrep::run(config)
}
