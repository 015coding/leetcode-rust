struct Solution;

impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut ans: Vec<String> = Vec::new();
        let _arr: Vec<i32> = (1..=n).collect();
        let mut check = 0;
        let mut idx = 0;
        let mut start = 1;

        loop {
            if check == target.len() {
                break;
            }

            if target[idx] == start {
                ans.push("Push".to_string());
                check += 1;
                idx += 1;
            } else {
                ans.push("Push".to_string());
                ans.push("Pop".to_string());
            }
            start += 1;
        }

        ans
    }
}

fn main() {
    println!("{:?}", Solution::build_array(vec![1, 2], 4));
}
