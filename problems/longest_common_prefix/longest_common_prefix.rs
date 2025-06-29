struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut longest_common_prefix = String::from("");
        let mut i: usize = 0;
        'index_loop: loop {
            let mut current_letter: Option<char> = None;
            for elem in &strs {
                let possible_next_letter = elem.chars().nth(i);
                match current_letter {
                    None => match possible_next_letter {
                        None => break 'index_loop,
                        Some(letter) => current_letter = Some(letter),
                    },
                    Some(letter) => match possible_next_letter {
                        None => break 'index_loop,
                        Some(next_letter) => {
                            if next_letter != letter {
                                break 'index_loop;
                            }
                        }
                    },
                }
            }
            match current_letter {
                None => break 'index_loop,
                Some(letter) => longest_common_prefix.push(letter),
            }
            i = i + 1
        }
        longest_common_prefix
    }
}

fn main() {
    let strs = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];
    println!("{}", Solution::longest_common_prefix(strs));
    let strs = vec![
        String::from("dog"),
        String::from("racecar"),
        String::from("car"),
    ];
    println!("{}", Solution::longest_common_prefix(strs));
}
