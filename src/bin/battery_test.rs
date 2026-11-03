struct Solution;
impl Solution {
    pub fn count_tested_devices(battery_percentages: Vec<i32>) -> i32 {
        let mut device: i32 = 0 ;
        for i in 0..battery_percentages.len() {
            if battery_percentages[i] <= device || battery_percentages[i] == 0 {
                continue;
            }else{
                device +=1 ;
            }
        }        
        device
    }
}

fn main() {
    println!("{}" , Solution::count_tested_devices([0,1,2].to_vec()));
}