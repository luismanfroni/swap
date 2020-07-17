use clap::Clap;

#[derive(Clap)]
#[clap(version = "1.0", author = "Luis Felipe Manfroni <luis.manfroni@catolicasc.org.br>")]
pub struct Opts {
    #[clap(subcommand)]
    pub command: Command
}

#[derive(Clap)]
pub enum Command {
    #[clap(version = "1.0", author = "Luis Felipe Manfroni <luis.manfroni@catolicasc.org.br>")]
    New(FileTarget),
    #[clap(version = "1.0", author = "Luis Felipe Manfroni <luis.manfroni@catolicasc.org.br>")]
    Checksum(FileTarget),
    #[clap(version = "1.0", author = "Luis Felipe Manfroni <luis.manfroni@catolicasc.org.br>")]
    Compress(DualFileTarget)
}

#[derive(Clap)]
pub struct FileTarget {
    pub input: String,
    /// Print debug info
    #[clap(short, long)]
    pub debug: bool
}

#[derive(Clap)]
pub struct DualFileTarget {
    pub input: String,
    pub second_input: String,
    /// Print debug info
    #[clap(short, long)]
    pub debug: bool
}

pub fn app() -> Opts {
    Opts::parse()
}