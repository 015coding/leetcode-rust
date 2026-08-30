struct Solution;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut count: i32 = 0;
        let ed = nums.len();
        for i in 1..ed + 1 {
            if nums[i - 1] == 0 {
                count = 0;
            } else {
                count += 1;
            }

            if count >= ans {
                ans = count;
            }
        }
        ans
    }
}
fn main() {
    println!(
        "{:?}",
        Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1, 1, 1])
    );
}

// ans 2
