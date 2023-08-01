use clap::Parser;

/// Space Game Server
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to json file with map configuration
    #[arg(long, default_value_t = String::from("maps/example.json"))]
    path: String,

    /// Port to bind server
    #[arg(long, default_value_t = 8888)]
    port: u16,
}

fn main() {
    let args = Args::parse();

    if let Err(e) = server::run(&args.path, args.port) {
        eprintln!("Server error: {e}");
        std::process::exit(1);
    }
}
