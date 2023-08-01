use clap::Parser;

/// Space Game Server
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to json file with map configuration
    #[arg(short, long, default_value_t = String::from("maps/example.json"))]
    path: String,

    /// Ip address to bind
    #[arg(short, long, default_value_t = String::from("0.0.0.0:8888"))]
    addr: String,
}

fn main() {
    let args = Args::parse();

    if let Err(e) = server::run(&args.path, &args.addr) {
        eprintln!("Server error: {e}");
        std::process::exit(1);
    }
}
