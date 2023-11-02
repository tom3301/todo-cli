use crate::task::parse_input;
use crate::task::Task;

pub fn get_tasks(input : &str) -> Vec<Task>{
    let mut list_of_tasks : Vec<Task> = vec![];
    for line in input.lines(){
        let one_task_as_struct = parse_input(line);
        list_of_tasks.push(one_task_as_struct);
    }
    list_of_tasks
}
