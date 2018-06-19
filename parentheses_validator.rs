
use std::io::{self, Read};

/// This is the program used for validating if the brackets (), [], {} are properly balanced.
/// Done for the purpose of Coursera "Computer Science: Algorithms, Theory, and Machines".


fn validate_parentheses(text: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for item in text.chars() {
        if let Some(stack_element) = stack.pop() {
            match (stack_element, item) {
                ('(', ')') => { continue },
                ('[', ']') => { continue },
                ('{', '}') => { continue },
                _ => { stack.push(stack_element) },
            }
        }
        stack.push(item);
    }
    stack.is_empty()
}

fn read_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    Ok(buffer)
}

fn main() {
    let text: String = read_input().unwrap().trim().to_string();
    println!("Result: {}", text);
    let result = validate_parentheses(&text);
    match result {
        true => println!("valid!"),
        false => println!("error!"),
    }
}


#[test]
fn it_validates_parentheses_ok() {
    let result = validate_parentheses("()");
    assert_eq!(result, true);
}

#[test]
fn it_validates_parentheses_left_error() {
    let result = validate_parentheses("((");
    assert_eq!(result, false);
}

#[test]
fn it_validates_parentheses_right_error() {
    let result = validate_parentheses("))");
    assert_eq!(result, false);
}

#[test]
fn it_validates_parentheses_three_error() {
    let result = validate_parentheses("())");
    assert_eq!(result, false);
}

#[test]
fn it_validates_parentheses_four_ok() {
    let result = validate_parentheses("(())");
    assert_eq!(result, true);
}

#[test]
fn it_validates_parentheses_neighbours_ok() {
    let result = validate_parentheses("()()");
    assert_eq!(result, true);
}

#[test]
fn it_validates_multiple_brackets_ok() {
    let result = validate_parentheses("([{}][{}])");
    assert_eq!(result, true);
}

#[test]
fn it_validates_multiple_brackets_error() {
    let result = validate_parentheses("([{}][{])");
    assert_eq!(result, false);
}
