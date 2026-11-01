struct Solution ;
impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let part1: Vec<char> = word1.chars().collect();
        let part2: Vec<char> = word2.chars().collect();
        let mut ans= String::new(); 

        for i in 0..word1.len().max(word2.len()) {
            if let Some(ch) = part1.get(i) {
                ans.push(*ch);
            }
            if let Some(ch) = part2.get(i) {
                ans.push(*ch);
            }
        } 
        ans.split_whitespace().collect()
    }
}

fn main() {
    println!("{}" , Solution::merge_alternately("a b c".to_string(), " d f".to_string()))
}



