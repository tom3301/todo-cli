use clap::Parser;
use clap::Subcommand;
/// Todo-cli
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct TodoArgs {
    /// Command to perform
    #[clap(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Add a task
    Add {
        input : String
    } ,
    /// Show all the tasks.
    Show{
        filter : Option<String>
    },
    /// Find a task containing string
    Find{
        find_task : String
    },
    /// Remove a task equal to string
    Remove {
        remove_task: String
    }
}