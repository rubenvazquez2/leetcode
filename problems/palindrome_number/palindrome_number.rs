use std::collections::VecDeque;

struct Solution {
}

impl Solution {
    pub fn is_palindrome(&self, x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let mut x_copy: i32 = x;
        let mut vec = VecDeque::new();

        while x_copy != 0 {
            vec.push_back(x_copy % 10);
            x_copy = x_copy / 10;
        }

        while vec.len() > 0 {
            let begin = vec.pop_front();
            if vec.len() == 0 {
                return true;
            }
            let end = vec.pop_back();

            if begin != end {
                return false;
            }
        }

        return true;
    }
}

fn main() {
    let solution: Solution = Solution {};

    let x: i32 = -121;
    let palindrome_number = solution.is_palindrome(x);
    println!("{:?}", palindrome_number);
}
