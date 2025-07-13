pub struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut w_size: usize = s.len();

        while w_size != 0 {
            let palindrome = Solution::get_palindrome_of_wsize(&s[..], w_size);
            match palindrome {
                Some((start_ind, end_ind)) => {
                    return String::from(&s[start_ind..end_ind]);
                }
                None => w_size = w_size - 1usize,
            }
        }

        String::from("")
    }

    fn get_palindrome_of_wsize(s: &str, w_size: usize) -> Option<(usize, usize)> {
        let mut start: usize = 0;

        while start + w_size - 1usize < s.len() {
            let is_palindrome = Solution::is_palindrome(&s[start..(start + w_size)]);
            if is_palindrome {
                return Some((start, start + w_size));
            }
            start = start + 1;
        }
        None
    }

    fn is_palindrome(s: &str) -> bool {
        let mut start = 0;
        let mut end = s.len() - 1usize;
        let mut is_palindrome = true;

        while end > start {
            let start_char = s.chars().nth(start);
            let end_char = s.chars().nth(end);

            match (start_char, end_char) {
                (Some(start_char), Some(end_char)) => {
                    if start_char == end_char {
                        start = start + 1usize;
                        end = end - 1usize;
                        continue;
                    } else {
                        is_palindrome = false;
                        break;
                    }
                }
                _ => {
                    is_palindrome = false;
                    break;
                }
            }
        }
        is_palindrome
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = Solution::longest_palindrome(String::from("babad"));
        assert_eq!(result, String::from("bab"));
    }

    #[test]
    fn example_2() {
        let result = Solution::longest_palindrome(String::from("cbbd"));
        assert_eq!(result, String::from("bb"));
    }

    #[test]
    fn example_3() {
        let result = Solution::longest_palindrome(String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabcaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"));
        assert_eq!(result, String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"));
    }
}
