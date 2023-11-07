use std::collections::HashMap;
use std::cmp::max;

fn length_of_longest_substring(s: String) -> i32 {
    let mut m = HashMap::new();
    let mut ans = 0;
    let mut before = -1;
    let mut current = 0;
    for c in s.chars() {
        if let Some(last) = m.insert(c, current) {
            before = max(before, last);
        }
        ans = max(ans, current - before);
        current += 1;
    }
    ans
}


fn main() {
    // let a = String::from("taaaa");
    // let mut b: Vec<u8> = a.as_bytes().into();
    // b.dedup();
    // println!("{:?}",b);
    // test 5

    let s = String::from("abcabcbb");
    println!("{}",length_of_longest_substring(s));
}