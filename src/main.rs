use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!(
        "Pesquisando por {} no arquivo {}.",
        config.query, config.filename
    );

    println!("No arquivo {}", config.filename);

    let contents = fs::read_to_string(config.filename).expect("Algo errado na leitura do arquivo.");

    println!("Existe o conteúdo:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}
