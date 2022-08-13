use std::io;

pub fn solve() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let v: Vec<&str> = s.split_whitespace().collect();
    let nums: Vec<i32> = v.iter().map(|x| x.parse::<i32>().unwrap()).collect();
    let mut result: String = String::new();

    let x = nums[0];

    let mut flag: bool = true;
    let mut cnt: i32 = 0;

    for i in &nums[1..] {
        if flag {
            for _ in 0..*i {
                if cnt == x {
                    result.push('\n');
                    cnt = 0;
                }

                result.push('0');
                cnt += 1;
            }
            flag = !flag;
        } else {
            for _ in 0..*i {
                if cnt == x {
                    result.push('\n');
                    cnt = 0;
                }

                result.push('1');
                cnt += 1;
            }
            flag = !flag;
        }
    }

    println!("{}", result);
}