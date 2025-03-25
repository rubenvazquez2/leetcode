use std::collections::HashMap;

struct Solution {
}

impl Solution {
    pub fn roman_to_int(&self, s: String) -> i32 {
        let mut i = 0;
        let mut val = 0;
        let size = s.len();

        let mut roman_to_int_map = HashMap::new();
        roman_to_int_map.insert('I', 1);
        roman_to_int_map.insert('V', 5);
        roman_to_int_map.insert('X', 10);
        roman_to_int_map.insert('L', 50);
        roman_to_int_map.insert('C', 100);
        roman_to_int_map.insert('D', 500);
        roman_to_int_map.insert('M', 1000);

        while i < size {
            let first_letter = s.chars().nth(i).unwrap();
            if i == size-1 {
                val = val + roman_to_int_map.get(&first_letter).unwrap();
                break;
            }
            match first_letter {
                'I' => {
                    let second_letter = s.chars().nth(i+1).unwrap();
                    if second_letter == 'V' || second_letter == 'X' {
                        val = val + (roman_to_int_map.get(&second_letter).unwrap() - roman_to_int_map.get(&first_letter).unwrap());
                        i = i + 1;
                    }
                    else {
                        val = val + roman_to_int_map.get(&first_letter).unwrap();
                    }
                    i = i + 1;
                },
                'X' => {
                    let second_letter = s.chars().nth(i+1).unwrap();
                    if second_letter == 'L' || second_letter == 'C' {
                        val = val + (roman_to_int_map.get(&second_letter).unwrap() - roman_to_int_map.get(&first_letter).unwrap());
                        i = i + 1;
                    }
                    else {
                        val = val + roman_to_int_map.get(&first_letter).unwrap();
                    }
                    i = i + 1;
                },
                'C' => {
                    let second_letter = s.chars().nth(i+1).unwrap();
                    if second_letter == 'D' || second_letter == 'M' {
                        val = val + (roman_to_int_map.get(&second_letter).unwrap() - roman_to_int_map.get(&first_letter).unwrap());
                        i = i + 1;
                    }
                    else {
                        val = val + roman_to_int_map.get(&first_letter).unwrap();
                    }
                    i = i + 1;
                },
                _ => {
                    val = val + roman_to_int_map.get(&first_letter).unwrap();
                    i = i + 1;
                }
            }
        }
        val
    }
}

fn main() {
    let solution = Solution {};
    println!("{}", solution.roman_to_int(String::from("IV")));
}
