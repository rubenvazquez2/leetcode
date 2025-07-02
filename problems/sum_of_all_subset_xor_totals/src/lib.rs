pub struct Solution {}

impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        Solution::subset_xor_sum_recurse(nums, 0)
    }

    pub fn subset_xor_sum_recurse(nums: Vec<i32>, idx: usize) -> i32 {
        if nums.len() == 0 {
            0
        } else if idx == nums.len() {
            let mut answer = 0;
            for num in nums {
                answer = answer ^ num;
            }
            answer
        } else {
            let mut nums_clone = nums.clone();
            nums_clone.remove(idx);
            Solution::subset_xor_sum_recurse(nums, idx + 1)
                + Solution::subset_xor_sum_recurse(nums_clone, idx)
        }
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
