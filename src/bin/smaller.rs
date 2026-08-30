struct Solution;

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::new();
        for num_i in nums.as_slice() {
            let mut count = 0;
            for num_j in nums.as_slice() {
                if num_i > num_j {
                    count += 1;
                }
            }
            ans.push(count);
        }

        ans
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::smaller_numbers_than_current(vec![8, 1, 2, 2, 3])
    );
}
