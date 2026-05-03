use std::env;
use std::process;

use Phone_Number_Manager::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Erreur de configuration : {}", err);
        process::exit(1);
    });

    if let Err(e) = Phone_Number_Manager::run(config) {
        eprintln!("Erreur : {}", e);
        process::exit(1);
    }
}