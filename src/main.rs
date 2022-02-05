use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_config(&args);

    println!("Pesquisando por {} no arquivo {}.", query, filename);

    println!("No arquivo {}", filename);

    let contents = fs::read_to_string(filename).expect("Algo errado na leitura do arquivo.");

    println!("Existe o conteúdo:\n{}", contents);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}
