use crate::task::{Task, parse_input};
use colored::*;

pub fn pretty_string(input : &Task) -> String{

   let words :Vec<&str> =  (input.task).split_whitespace().collect();
   let mut beautiful_print :String = "".to_string();

   for word in words.iter(){
        let word_copy = word.to_string();
        let len = word_copy.len();

        if input.tags.contains(&word_copy[1..len].to_string()){ // make the word green if it is a tag
            beautiful_print.push_str(format!("{}",word.green()).as_str());
            beautiful_print.push_str(" ")
        }
        else if input.projects.contains(&word_copy[1..len].to_string()){ // make the word red if it is a project
            beautiful_print.push_str(format!("{}",word.red()).as_str());
            beautiful_print.push_str(" ")
        } else{
            beautiful_print.push_str(word);
            beautiful_print.push_str(" ");
        }}
        beautiful_print

}
pub fn pretty_show_tasks (tasks: Vec<Task>,filter_word: Option<String>){
    // given a string containing multiple tasks, pretty_show the task
    match &filter_word{
        Some(_filter)=>{
            let filter = filter_word.unwrap();
            for (index,task) in tasks.iter().enumerate(){
                if task.task.contains(&filter){
                    println!("({}) {}",(index+1).to_string().blue(), pretty_string(task))
                }
            }

        }
        None=>{
            for (index,task) in tasks.iter().enumerate(){
                println!("({}) {}",(index+1).to_string().blue(), pretty_string(task))
            }
        }
    }
    
}