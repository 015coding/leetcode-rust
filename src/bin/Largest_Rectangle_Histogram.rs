struct Solution;
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut ans = 0 ;
        let mut stack : Vec<usize> = Vec::new();
        for i in 0..=heights.len() {
            let current = if i < heights.len() {heights[i]} else {0};
            while let Some(&top) = stack.last() {
                if heights[top] <= current {
                    break;
                }
                let index = stack.pop().unwrap();
                let h = heights[index];
                let width = match stack.last() {
                    Some(&left) => i - left - 1,
                    None => i,
                };
                ans = ans.max(h * width as i32)

            }
            stack.push(i);
        
        }
        ans

    }
}

fn main() {
    println!("{}" , Solution::largest_rectangle_area([2,4].to_vec()));
}