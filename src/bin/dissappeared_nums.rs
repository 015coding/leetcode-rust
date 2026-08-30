struct Solution;

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::new();
        let n = nums.len();
        let mut count = vec![0; n + 1];

        for num in nums {
            count[num as usize] += 1;
        }

        for i in 1..=n {
            if count[i] == 0 {
                ans.push((i) as i32);
            }
        }

        ans
    }
}

fn main() {
    println!("{:?}", Solution::find_disappeared_numbers(vec![1, 1]));
}
