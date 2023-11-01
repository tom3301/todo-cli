use crate::task::{Task, parse_input};
use colored::*;

pub fn pretty_show_task(input : Task){
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
    println!("{beautiful_print}");

}
pub fn pretty_show_tasks (contents: &str){
    // given a string containing multiple tasks, pretty_show the task
    for line in contents.lines(){
        pretty_show_task(parse_input(line))
    }
}