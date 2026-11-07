struct Solution;
impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0 ; 
        let mut idx = 0 ;
        let mut h = tickets.clone();
        while h[k as usize] != 0 {
            if h[idx] != 0 {
                h[idx] -= 1;
                ans += 1;
            }
            idx = (idx + 1) % h.len();
        }
        ans
    }
}

fn main() {
    let tickets = [5,1,1,1].to_vec();
    let k = 0;
    println!("{}" , Solution::time_required_to_buy(tickets , k));
}