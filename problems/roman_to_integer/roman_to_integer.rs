use std::collections::HashMap;

struct Solution {
}

impl Solution {
    pub fn roman_to_int(&self, s: String) -> i32 {
        let mut i = 0;
        let mut val = 0;

        let mut roman_to_int_map = HashMap::new();
        roman_to_int_map.insert('I', 1);
        roman_to_int_map.insert('V', 5);
        roman_to_int_map.insert('X', 10);
        roman_to_int_map.insert('L', 50);
        roman_to_int_map.insert('C', 100);
        roman_to_int_map.insert('D', 500);
        roman_to_int_map.insert('M', 1000);

        loop {
            let first_letter = s.chars().nth(i);
            match first_letter {
                None => break,
                Some(first_letter) => {
                    let first_letter_val = roman_to_int_map.get(&first_letter);
                    match first_letter_val {
                        None => panic!("Unexpected first character {}", first_letter),
                        Some(first_letter_val) => {
                            i = i + 1;
                            let second_letter = s.chars().nth(i);
                            match second_letter {
                                None => val = val + first_letter_val,
                                Some(second_letter) => {
                                    let second_letter_val = roman_to_int_map.get(&second_letter);
                                    match second_letter_val {
                                        None => panic!("Unexpected second character {}", second_letter),
                                        Some(second_letter_val) => {
                                            match (first_letter, second_letter) {
                                                ('I', 'V' | 'X') | ('X', 'L' | 'C') | ('C', 'D' | 'M' ) => {
                                                    val = val + (second_letter_val - first_letter_val);
                                                    i = i + 1;
                                                },
                                                _ => val = val + first_letter_val,
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        val
    }
}

fn main() {
    let solution = Solution {};
    println!("{}", solution.roman_to_int(String::from("MCMXCIV")));
}
