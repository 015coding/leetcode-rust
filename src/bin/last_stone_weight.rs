struct Solution;
impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut new_stones: Vec<i32> = stones.clone();
        let ans: i32 ; 
        
        while new_stones.len() >= 2{
            new_stones.sort();
            let y: i32 = new_stones.pop().unwrap();
            let x: i32 = new_stones.pop().unwrap();
            if x != y {
                new_stones.push(y-x);
            }
        } 
        if new_stones.is_empty() {
            ans = 0;
        }else{
            ans = new_stones.pop().unwrap();
        }
        ans
    }
}
fn main() {
    let stones: Vec<i32> = [1].to_vec();
    println!("{}", Solution::last_stone_weight(stones));
}