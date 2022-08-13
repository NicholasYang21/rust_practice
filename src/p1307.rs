use std::io;

fn solve() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Panic.");

    if s.as_bytes()[0] == '-' as u8 {
        print!("-");
        s.remove(0);
    }

    if s.as_bytes()[0] == '0' as u8 {
        println!("0");
        return;
    }

    let mut num: bool = true;

    for i in s.chars().rev().filter(|x| x.is_ascii_graphic()) {
        if i == '0' && num { continue; }
        else {
            num = false;
            print!("{}", i);
        }
    }
}