use clap::Clap;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap)]
#[clap(version = "1.0", author = "Luis Felipe Manfroni <luis.manfroni@catolicasc.org.br>")]
pub struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short, long, default_value = "default.conf")]
    pub config: String,

    /// Some input. Because this isn't an Option<T> it's required to be used
    #[clap()]
    pub input: String,
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: i32,
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {
    #[clap(version = "1.3", author = "Someone E. <someone_else@other.com>")]
    Test(Test),
}

/// A subcommand for controlling testing
#[derive(Clap)]
pub struct Test {
    /// Print debug info
    #[clap(short)]
    pub debug: bool
}

pub fn app() -> Opts {
    return Opts::parse();
}