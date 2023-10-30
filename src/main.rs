use clap::Parser;
use args::TodoArgs;
use std::io;

mod args;
mod file_func;
fn main()-> io::Result<()> {
// Get the command from the cli
    let mut args: TodoArgs = TodoArgs::parse();

// Write task to file only if told
    match args.task.as_str() {
        " " => (),
        s => {
            let _ = file_func::write_to_file(s);
            println!{"Added task: {}",s}
            ()}
    }
    match args.find_task.as_str(){
        " " => (),
        s => {
            args.show = 0;
            let _ = file_func::find_task(s);
            ()}
    }

// Show tasks only if told (Default=1)
    match args.show {
        1 => file_func::show_tasks()?,
        _ => () }

    match args.remove_task.as_str(){
        " " => (),
        s => {
            args.show = 0;
            let _ = file_func::remove_task(s);
            ()}}


    Ok(())

}
