use clap::Parser;
use chrono::Utc;

#[derive(Parser)]
struct Cli {
    timezone: String,
}

fn main() {
    let dt = Utc::now();
    println!("{}", dt);
}
