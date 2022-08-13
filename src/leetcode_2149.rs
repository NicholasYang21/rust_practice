
pub fn solve() {
    let in_: Vec<i32> = Vec::from([3, 1, -2, -5, 2, -4]);

    let mut res: Vec<i32> = Vec::new();

    let mut po: Vec<i32> = Vec::new();
    let mut n: Vec<i32> = Vec::new();

    for i in &in_ {
        if i.is_positive() { po.push(*i); }
        if i.is_negative() { n.push(*i); }
    }

    for i in 0..in_.len() {
        let p = i;
        if i % 2 == 1 {
            res.push(n[p / 2]);
        } else {
            res.push(po[p / 2]);
        }
    }

    println!("{:?}", res);
}