// Problem 1: Fix the code so that it compiles.


fn first_character(chars: &Vec<char>) -> Option<char> {
    if chars.len() > 0 {
        Some(chars[0])
    } else {
        None
    }
}

fn main() {
    let my_chars = vec!['a', 'b', 'c', 'd'];
    if let Some(character) = first_character(&my_chars) {
        println!("First character: {character}")
    } else {
        println!("Empty array")
    }
}


/*
Option is automatically imported and is an enum

enum Option<T> {
    None,
    Some(T)
}

*/