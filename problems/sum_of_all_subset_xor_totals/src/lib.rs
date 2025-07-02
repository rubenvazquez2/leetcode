pub struct Solution {}

impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::subset_xor_sum(vec![1, 3]), 6);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::subset_xor_sum(vec![5, 1, 6]), 28);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::subset_xor_sum(vec![3, 4, 5, 6, 7, 8]), 480);
    }
}
