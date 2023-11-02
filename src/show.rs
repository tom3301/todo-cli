use crate::get_tasks::get_tasks;
use crate::pretty_show::*;
use crate::file_func::*;
use crate::io;

pub fn show_tasks(filter: &Option<String>) -> io::Result<()>{
    // Print the todo.txt file
    let contents: Result<String, io::Error> = read_todo_file();
    match contents{
        Ok(content) =>{
            let tasks: Vec<crate::task::Task> = get_tasks(&content);
                
            match filter {
                Some(filter_word)=>{
                    pretty_show_tasks(tasks,Some(filter_word.to_string()));
                    Ok(())
                },
                None =>{
                    pretty_show_tasks(tasks,None);
                    Ok(())
                }
            }
        },
        Err(err)=>Err(err)
    }
}