struct Solution;

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut count = 0;
        let place = flowerbed.to_vec();
        let mut i = 0;
        while i < place.len() {
            let left = i == 0 || place[i-1] == 0 ;
            let right = i +1 == place.len() || place[i+1] == 0 ;
            if place[i] == 0 && left && right {
                count += 1 ;
                i += 2;
            }else{
                i += 1;
            }
        }
        if count >= n {
            true
        }else{
            false
        }
    }
}

fn main() {
    let flowerbed = [1,0,0,0,1].to_vec();
    let n = 2 ;
    println!("{}" , Solution::can_place_flowers(flowerbed, n));
}