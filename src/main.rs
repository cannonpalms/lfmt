use clap::Parser;

mod commands {
    pub mod get;
}

enum ReturnCode {
    Error = 1,
}

#[derive(Debug, clap::Parser)]
#[clap(name = "lfmt")]
struct Config {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, clap::Parser)]
enum Command {
    Get(commands::get::Config),
}

fn main() {
    let config: Config = Config::parse();
    match config.command {
        Command::Get(config) => {
            if let Err(e) = commands::get::run(config) {
                eprintln!("{e}");
                std::process::exit(ReturnCode::Error as _)
            }
        }
    }
}
