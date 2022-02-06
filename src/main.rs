use std::env;
use std::process;

use minigrep::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Ocorreu um problema: {}", err);
        process::exit(1)
    });

    println!(
        "Pesquisando por {} no arquivo {}.",
        config.query, config.filename
    );

    println!("No arquivo {}", config.filename);

    if let Err(e) = run(config) {
        println!("Erro na aplicação: {}", e);
        process::exit(1);
    }
}
