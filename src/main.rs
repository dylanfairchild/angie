use std::env;

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    
    let config = angie::Config::new(&args);

    angie::run(config).unwrap();
}
