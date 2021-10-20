mod serve;
mod task;

use crate::connector::ConnectorBuilder;
use clap::Parser;
use task::TaskFlags;

/// Tackode backend CLI - Serve API or execute task
#[derive(Parser, Debug)]
#[clap(version = "2.0.0", author = "Julien Blatecky")]
struct Opts {
    #[clap(subcommand)]
    main_command: MainCommand,
}

#[derive(Parser, Debug)]
enum MainCommand {
    /// Execute task like updating gauges - to be used with cron job
    #[clap(name = "task")]
    Task(TaskFlags),

    /// Serve API
    #[clap(name = "serve")]
    Serve,
}

pub async fn run(builder: ConnectorBuilder) {
    let opts = Opts::parse();

    match opts.main_command {
        MainCommand::Task(task_flags) => task::run(task_flags, builder).await,
        MainCommand::Serve => serve::run(builder).await,
    }
}
