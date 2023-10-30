use std::fs::File;
use std::io::{self, Read};
use std::fs::OpenOptions;
use std::io::Write;
const FILE_PATH : &str = "todo.txt";



pub fn read_todo_file()-> Result<String,io::Error>{
    // read the todo.txt file
    let mut file = File::open(FILE_PATH)?;
    // Create a buffer to hold the file's contents
    let mut contents = String::new();
    // Read the file's contents into the buffer
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn write_to_file(task: &str) -> io::Result<()> {
    // write the given task to the file on a new line
    //
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(FILE_PATH)?;

    // Write the data to the file
    // file.write_all(b"\n")?;
    file.write_all(task.as_bytes())?;

    Ok(())

}

pub fn show_tasks() -> io::Result<()>{
    // Print the todo.txt file
    let contents = read_todo_file();
    match contents {
        Ok(s) => {
            println!("Tasks:\n{}", s);
            Ok(())},
        Err(err) => {
            println!("Error; cannot read file");
            Err(err)}
    }
}
pub fn find_task(task: &str) -> io::Result<()>{
    let contents = read_todo_file();
    println!("Tasks containing {}:\n",task);

    for line in contents?.lines(){
        if line.contains(task){
            println!("{}",line.to_string());
        }else{}
    }
    Ok(())
}
pub fn remove_task(task:&str) -> io::Result<()>{

    // Open the file for reading
    let contents = read_todo_file();

    // Create a temporary file to write the modified contents
    let temp_file_path = "temp_file.txt";
    let temp_file = File::create(&temp_file_path)?;

    // Iterate through the lines and skip the line to be removed
    for line in contents?.lines() {
        if line != task {
            writeln!(&temp_file, "{}", line)?;
        } else {println!("Removed task :{}", line)}
    }
    // Close the temporary file
    drop(temp_file);

    // Replace the original file with the temporary file
    std::fs::rename(temp_file_path, FILE_PATH)?;

    Ok(())
}
