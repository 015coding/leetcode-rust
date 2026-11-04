struct Solution;
impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut ans = [0,0];
        for student in students {
            ans[student as usize] += 1;
        }

        for sandwich in sandwiches {
            if ans[sandwich as usize] == 0 {
                break;
            }
            ans[sandwich as usize] -=1
            
        }
        ans[0] + ans[1]

    }
}

fn main() {
    println!("{}" , Solution::count_students([1,1,1,0,0,1].to_vec(), [1,0,0,0,1,1].to_vec()))
}