mod cli;

fn main() {
    if let Err(code) = cli::run(std::env::args().collect()) {
        std::process::exit(code);
    }
}
