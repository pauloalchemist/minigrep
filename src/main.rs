use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Ocorreu um problema com os argumentos: {}", err);
        process::exit(1)
    });

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
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Argumentos insuficientes para realizar a operação.");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
