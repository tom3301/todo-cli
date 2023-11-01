use regex::Regex;

pub struct Task{
    pub task : String,
    pub priority : char,
    pub projects : Vec<String>,
    pub tags : Vec<String>,
}


fn find_projects(input: &str, result : &mut Task){
    // find the projects in the string starting with % and add them to the vector

    let projects = Regex::new(r"\+\w+").unwrap(); //TODO make sure that _ does not break the word
    // Iterate over matched words and print them

    // find all the projects and add them to the vector
    for word in projects.find_iter(input) {
        let len = word.len();
        let project = &word.as_str()[1..len];
        result.projects.push(project.to_string());
    }
}

fn find_tags(input: &str, result : &mut Task){
    // find the projects in the string starting with % and add them to the vector

    let tags = Regex::new(r"@\w+").unwrap(); //TODO make sure that _ does not break the word
    // Iterate over matched words and print them

    // find all the projects and add them to the vector
    for word in tags.find_iter(input) {
        let len = word.len();
        let tag = &word.as_str()[1..len];
        result.tags.push(tag.to_string());
    }
}

fn find_priority(input: &str, result : &mut Task){
    // find the projects in the string starting with % and add them to the vector

    // Iterate over matched words and print them
    let re = Regex::new(r"^\([A-Z]\)").unwrap();
    // find all the projects and add them to the vector
    for letter in re.find_iter(input) {
        result.priority = letter.as_str().chars().nth(1).unwrap();
    }
}
pub fn parse_input(input:&str)->Task {

    let mut result = Task{
        task : input.to_string(),
        priority : 'n',
        projects : vec![],
        tags : vec![],
    };

    find_projects(input, &mut result);
    find_tags(input, &mut result);
    find_priority(input, &mut result);


    result
}

