use std::io;

fn solve() {
    let mut line = String::new();
    let mut result: usize = 0;

    match io::stdin().read_line(&mut line) {
        Ok(_) => (),
        Err(err) => println!("Error: {err}"),
    }

    
    let v: Vec<&str> = line.split(' ').collect();

    let l = v[0]
        .to_string()
        .trim()
        .parse::<i32>()
        .unwrap();

    let r = v[1]
        .to_string()
        .trim()
        .parse::<i32>()
        .unwrap();

    for i in l ..= r {
        result += i.to_string().matches('2').count();
    }

    println!("{}", result);
}