use clap::Parser;

/// Space Game
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to json file with map configuration
    #[arg(long, default_value_t = String::from("maps/example.json"))]
    path: String,

    /// Port to bind server
    #[arg(long, default_value_t = 8888)]
    port: u32,
}

fn main() {
    let args = Args::parse();

    if let Err(e) = space_game::run(&args.path, args.port) {
        println!("Application error: {e}");
        std::process::exit(1);
    }
}
