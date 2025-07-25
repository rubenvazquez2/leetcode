pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();
        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut nums = vec![1, 2, 2];
        let expected = vec![1, 2];

        let k = Solution::remove_duplicates(&mut nums);

        assert_eq!(expected.len() as i32, k);
        for i in 0..k {
            assert_eq!(nums[i as usize], expected[i as usize]);
        }
    }

    #[test]
    fn example_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let expected = vec![0, 1, 2, 3, 4];

        let k = Solution::remove_duplicates(&mut nums);

        assert_eq!(expected.len() as i32, k);
        for i in 0..k {
            assert_eq!(nums[i as usize], expected[i as usize]);
        }
    }
}
