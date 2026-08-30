struct Solution;

impl Solution {
    pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        let mut ans: Vec<usize> = vec![0; n as usize];
        let mut stack: Vec<usize> = Vec::new();
        let mut prev_t = 0;

        for log in logs {
            let parts: Vec<&str> = log.split(":").collect();
            let id = parts[0].parse::<usize>().unwrap();
            let status = parts[1].to_string();
            let time = parts[2].parse::<usize>().unwrap();

            if status == "start" {
                if let Some(&current) = stack.last() {
                    ans[current as usize] += time - prev_t - 1;
                }
                stack.push(id);
                prev_t = time;
            } else {
                // println!("{:?} => {:?} " , id , time - prev_t);
                ans[id] += time - prev_t + 1;
                stack.pop();
                prev_t = time;
            }
        }
        ans.into_iter().map(|x| x as i32).collect()
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::exclusive_time(
            2,
            vec![
                "0:start:0",
                "0:start:2",
                "0:end:5",
                "1:start:6",
                "1:end:6",
                "0:end:7"
            ]
            .into_iter()
            .map(String::from)
            .collect()
        )
    )
}
