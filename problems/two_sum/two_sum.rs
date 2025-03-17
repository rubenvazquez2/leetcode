struct Solution {
}

impl Solution {
    pub fn two_sums(&self, nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut o_counter: usize = 0;

        loop {
            let mut i_counter: usize = o_counter + 1;

            loop {
                let possible_two_sum = &nums[o_counter] + &nums[i_counter];

                if possible_two_sum == target {
                    let mut v: Vec<i32> = Vec::new();
                    v.push(o_counter as i32);
                    v.push(i_counter as i32);
                    return v;
                }

                i_counter = i_counter + 1;

                if i_counter > nums.len()-1 {
                    break;
                }
            }

            o_counter = o_counter + 1;
        }
    }
}

fn main() {
    let solution: Solution = Solution {};

    let nums = vec![1, 2, 3, 4, 5];
    let target = 4;
    let two_sums = solution.two_sums(nums, target);
    println!("{:?}", two_sums.get(0));
    println!("{:?}", two_sums.get(1));
}
