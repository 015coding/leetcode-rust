struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut ans = 0;
        let mut prev: i32 = 0;
        for item in s.chars().rev() {
            let value = match item {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            };

            if value >= prev {
                ans += value;
            } else {
                ans -= value;
            }
            prev = value;
        }
        ans
    }
}

fn main() {
    println!("{:?}", Solution::roman_to_int("MCMXCIV".to_string()))
}
