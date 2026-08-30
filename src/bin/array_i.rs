struct Solution;

impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let combine = [nums.as_slice(), nums.as_slice()].concat();
        combine
    }
}

fn main() {
    println!("{:?}", Solution::get_concatenation(vec![1, 2, 1]));
}

// ans [1,2,1,1,2,1]
