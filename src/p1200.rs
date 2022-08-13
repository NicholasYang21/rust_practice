use std::io;

fn solve() {
    let (mut st, mut sh) : (String, String) = (String::new(), String::new());
    io::stdin().read_line(&mut st).expect("Panic.");
    io::stdin().read_line(&mut sh).expect("Panic.");

    let (mut r1, mut r2): (i32, i32) = (1, 1);

    for i in st.chars() {
        r1 *= i as i32 - 'A' as i32 + 1;
    }

    for i in sh.chars() {
        r2 *= i as i32 - 'A' as i32 + 1;
    }

    if (r1 % 47) == (r2 % 47) {
        println!("GO");
    } else {
        println!("STAY");
    }
}