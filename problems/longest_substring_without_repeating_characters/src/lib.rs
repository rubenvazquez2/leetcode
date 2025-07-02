pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut answer = 0;
        let mut longest_substring = String::from("");

        for c in s.chars() {
            if longest_substring.contains(c) {
                let current_substring_length = longest_substring.len() as i32;
                if current_substring_length > answer {
                    answer = current_substring_length;
                }
                while longest_substring.contains(c) {
                    longest_substring.remove(0);
                }
            }
            longest_substring.push(c);
        }
        if longest_substring.len() as i32 > answer {
            longest_substring.len() as i32
        } else {
            answer
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let ex = String::from("abcabcbb");
        assert_eq!(Solution::length_of_longest_substring(ex), 3);
    }

    #[test]
    fn example_2() {
        let ex = String::from("bbbbb");
        assert_eq!(Solution::length_of_longest_substring(ex), 1);
    }

    #[test]
    fn example_3() {
        let ex = String::from("pwwkew");
        assert_eq!(Solution::length_of_longest_substring(ex), 3);
    }
}
