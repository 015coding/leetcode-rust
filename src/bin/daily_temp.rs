struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack : Vec<i32> = Vec::new();
        let n  = temperatures.len();
        let mut ans : Vec<i32> = vec![0 ; n] ;
        for idx in 1..n {
            stack.push(idx as i32 - 1);
            if temperatures[idx-1] < temperatures[idx] {
                loop {
                    let mut x: i32 = 0 ;
                    if let Some(&i) = stack.last() {
                        x = i ;
                    }
                    if stack.is_empty() || temperatures[x as usize] >= temperatures[idx] {
                        break;
                    }
                    ans[x as usize] = idx as i32 - x ;
                    stack.pop();
                }
            }

        }
        ans
    }
}


fn main() {
    println!("{:?}" , Solution::daily_temperatures(vec![73,74,75,71,69,72,76,73])) ;
}
