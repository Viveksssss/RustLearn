use minigrep::Config;
use std::env;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Probloem parsing arguments:{err}");
        std::process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        eprint!("Application error:{e}");
        std::process::exit(1);
    }
}
