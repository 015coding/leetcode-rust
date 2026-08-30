struct Solution;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut count = vec![0; n + 1];

        for num in nums {
            count[num as usize] += 1;
        }

        let mut dupicate = 0;
        let mut missing = 0;
        for i in 0..=n {
            if count[i] == 2 {
                dupicate = i as i32;
            }

            if count[i] == 0 {
                missing = i as i32;
            }
        }

        vec![dupicate, missing]
    }
}

fn main() {
    println!("{:?}", Solution::find_error_nums(vec![2, 3, 2]));
}
