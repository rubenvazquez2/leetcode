use std::vec::Vec;

struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        let mut valid = true;

        for c in s.chars() {
            match c {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                closing_parenthesis => {
                    let popped_parenthesis = stack.pop();
                    match popped_parenthesis {
                        None => {
                            valid = false;
                            break;
                        }
                        Some(parenthesis) => {
                            if closing_parenthesis != parenthesis {
                                valid = false;
                                break;
                            }
                        }
                    }
                }
            }
        }
        if stack.len() > 0 {
            valid = false;
        }
        valid
    }
}

fn main() {
    println!(
        "{} is valid {}",
        String::from("()"),
        Solution::is_valid(String::from("()"))
    );
    println!(
        "{} is valid {}",
        String::from("()[]{}"),
        Solution::is_valid(String::from("()[]{}"))
    );
    println!(
        "{} is valid {}",
        String::from("(]"),
        Solution::is_valid(String::from("(]"))
    );
    println!(
        "{} is valid {}",
        String::from("([])"),
        Solution::is_valid(String::from("([])"))
    );
    println!(
        "{} is valid {}",
        String::from("["),
        Solution::is_valid(String::from("["))
    );
}
