struct Solution;

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        let mut count = 0;
        let mut start = 0;
        let mut idx = start;
        while count < nums.len() {
            if idx >= nums.len() {
                start += 1;
                idx = start;
            }
            ans.push(nums[idx]);
            idx += n as usize;
            count += 1
        }

        ans
    }
}
fn main() {
    println!("{:?}", Solution::shuffle(vec![2, 5, 1, 3, 4, 7], 3));
}

// ans [2,3,5,4,1,7]
