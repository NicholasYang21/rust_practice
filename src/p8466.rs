fn solve() {
    let mut line = String::new();
    match std::io::stdin().read_line(&mut line) {
        Ok(_) => (),
        Err(err) => println!("Error: {err}"),
    }

    let t = line.trim();
    let num: i32 = t.parse::<i32>().unwrap();

    for _ in 0..num {
        line.clear();
        match std::io::stdin().read_line(&mut line) {
            Ok(_) => (),
            Err(err) => println!("Error: {err}"),
        }

        check_string(&line);
    }
}

fn check_string(str: &str) {
    if (str.contains('D')) && (str.contains('X')) {
        println!("Yes");
        return;
    }
    
    {
        for i in str.chars() {
            if str.matches(i).count() == 4 {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}