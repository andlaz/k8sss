use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(
    author = "Andras Szerdahelyi <andras.szerdahelyi@gmail.com>",
    version,
    about = "Poor man's service mesh"
)]
pub(crate) struct Cli {
    #[clap(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    Wait {
        #[clap(subcommand)]
        command: WaitCommands,
    },
}

#[derive(Subcommand)]
pub(crate) enum WaitCommands {
    Until {
        #[clap(subcommand)]
        until: WaitUntilCommands,
    },
}

#[derive(Subcommand)]
pub(crate) enum WaitUntilCommands {
    Service {
        #[clap(subcommand)]
        service: WaitUntilServiceCommands,
    },
    Job {
        #[clap(subcommand)]
        job: WaitUntilJobCommand,
    },
}

#[derive(Subcommand)]
pub(crate) enum WaitUntilServiceCommands {
    UnavailableEndpoints {
        #[clap(long)]
        lte: u32,
        name: String,
        namespace: Option<String>,
    },
    AvailableEndpoints {
        #[clap(long)]
        gte: u32,
        name: String,
        namespace: Option<String>,
    },
}

#[derive(Subcommand)]
pub(crate) enum WaitUntilJobCommand {
    Ready {
        name: String,
        namespace: Option<String>,
    },
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
