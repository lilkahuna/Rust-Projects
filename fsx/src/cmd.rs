use std::fs::{self, File};

pub fn search_file(query: &String, file: &String) -> usize {
    let content: String = fs::read_to_string(file).expect("Couldn't read file");

    // Token every word
    let words: Vec<&str> = content.split_whitespace().collect();
    let mut counter: usize = 0;
            
    for word in words {
        if word == query {
            counter += 1;
        }
    }
    counter
}

pub fn replace_in_file(to_replace: &String, with_replace: &String, file: &String) {
    let content: String = fs::read_to_string(file).expect("Coudln't read file");
    let mut new_content = String::new();
    
    for line in content.lines() {
        for word in line.split_whitespace() {
            if word == to_replace {
                new_content.push_str(format!("{with_replace} ").as_str());
            } else {
                new_content.push_str(format!("{word} ").as_str());
            }
        }
        new_content.push_str("\n");
    }
    
    fs::write(file, new_content).expect("Couldn't write to file");
}

fn get_file_size(file: &String) -> u64 {
    let mut opened_file = File::open(file).expect("Couldn't open file");
    let file_size = opened_file.metadata().expect("Couldn't get metadata").len();
    
    file_size
}

pub fn get_word_count(file: &String) -> usize {
    let content = fs::read_to_string(file).expect("Couldn't read file");
    let mut word_count: usize = 0;

    for _word in content.split_whitespace() {
        word_count += 1;
    }
    word_count
}

pub fn get_char_count(file: &String) -> usize{
    let content = fs::read_to_string(file).expect("Couldn't read file");
    let mut char_count: usize = 0;

    for word in content.split_whitespace() {
        char_count += word.len();
    }
    char_count
}
