struct Solution;

impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        let n = prices.len();

        for i in 0..n {
            let mut m = prices[i];
            let mut status: bool = false;
            for j in i + 1..n {
                if prices[j] <= m {
                    m = prices[j];
                    status = true;
                    break;
                }
                status = false;
            }
            if status {
                ans.push(prices[i] - m);
            } else {
                ans.push(prices[i]);
            }
        }

        ans
    }
}

fn main() {
    println!("{:?}", Solution::final_prices(vec![1, 2, 3, 4, 5]));
}
