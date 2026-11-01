struct Solution;
impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let greatest = candies.iter().max().unwrap();
        let mut ans : Vec<bool> = Vec::new();
        for i in 0..candies.len() {
            if (candies.get(i).unwrap() + extra_candies) >= *greatest {
                ans.push(true);
            }else{
                ans.push(false);
            }
        } 
        ans
    }
}

fn main() {
    let candies: Vec<i32> = [2,3,5,1,3].to_vec() ;
    let extra = 3;
    println!("{:?}" , Solution::kids_with_candies(candies, extra));
}