struct Solution {
}

impl Solution {
    pub fn create_target_array(&self, nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let mut vec = Vec::new();
        let mut i = 0;
        for idx in index.iter() {
            vec.insert(*idx as usize, nums[i]);
            i = i + 1;
        }
        return vec;
    }
}

fn main() {
    let nums = vec![0, 1, 2, 3, 4];
    let index = vec![0, 0, 2, 1, 3];
    let solution = Solution {};
    println!("{:?}", solution.create_target_array(nums, index));
}
