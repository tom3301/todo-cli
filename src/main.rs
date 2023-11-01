use clap::Parser;
use args::TodoArgs;
use std::io;

mod args;
mod file_func;
mod task;
mod pretty_show;


fn main()-> io::Result<()> {
// Get the command from the cli
    let args: TodoArgs = TodoArgs::parse();

    match args.command{
        //Show all the tasks
        Some(args::Command::Show {filter})=> file_func::show_tasks(&filter)?,
        // Add a task
        Some(args::Command::Add {input}) =>{
            // make a task struct out of the input
            file_func::write_to_file(&input)?;
            println!("Task added : {}",input)

        }
        // Remove a task
        Some(args::Command::Remove { remove_task}) =>file_func::remove_task(&remove_task)?,
        // Find a task
        Some(args::Command::Find { find_task}) =>file_func::find_task(&find_task)?,
        None => println!("Niet gelukt!")
    } 
    Ok(())

}

#[cfg(test)]
mod tests {
    use crate::task::parse_input;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(task::parse_input("test is %programma").task,"test is %programma");
    }
    #[test]
    fn test_parser_projects() {
        assert_eq!(task::parse_input("test is +programma").projects[0],"programma");
        assert_eq!(task::parse_input("test is +task_cli").projects[0],"task_cli");
        assert_eq!(task::parse_input("test is +programma en +taskcli").projects[1],"taskcli");
    }

    #[test]
    fn test_parser_tags() {
        assert_eq!(task::parse_input("test is %programma en een @tag").tags[0],"tag");
        assert_eq!(task::parse_input("test is %programma en een @tag @thuis").tags[1],"thuis");
    }

    #[test]
    fn test_parser_priority() {
        assert_eq!(task::parse_input("(A) test is %programma en een @tag").priority,'A');
        assert_eq!(task::parse_input("test (A) is %programma en een @tag").priority,'n'); //n if there is no priority
    }
    #[test]
    fn test_pretty_print() {
        pretty_show::pretty_show_task(parse_input("test de tag functie @deze en dit project +todo"));
    }

}