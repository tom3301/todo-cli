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
    file.write_all(b"\n")?;
    file.write_all(task.as_bytes())?;
    Ok(())

}

pub fn remove_task(line_number_to_del: u8) -> io::Result<()>{

    // Open the file for reading
    let contents = read_todo_file();

    // Create a temporary file to write the modified contents
    let temp_file_path = "temp_file.txt";
    let temp_file = File::create(&temp_file_path)?;

    // Iterate through the lines and skip the line to be removed
    for (index,line) in contents?.lines().enumerate() {
        if index + 1 != line_number_to_del as usize {
            writeln!(&temp_file, "{}", line)?;
        } else {println!("Removed task :{}", line)}
    }
    // Close the temporary file
    drop(temp_file);

    // Replace the original file with the temporary file
    std::fs::rename(temp_file_path, FILE_PATH)?;

    Ok(())
}
