use clap::Parser;

/// Todo-cli
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct TodoArgs {
    /// Show the tasks
    #[arg(short,default_value_t = 0)]
    pub show: u8,

    /// Task to add
    #[arg(short,default_value_t=String::from(" "))]
    pub task: String,

    /// Task to find
    #[arg(short,default_value_t=String::from(" "))]
    pub find_task: String,

    /// Task to remove
    #[arg(short,default_value_t=String::from(" "))]
    pub remove_task: String,
}
