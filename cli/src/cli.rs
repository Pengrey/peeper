use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_enum)]
    application: Application,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Application {
    Msedge,
    Chrome,
    Desktop,
}

pub fn get_choice() -> Application {
    let args = Args::parse();
    args.application
}
