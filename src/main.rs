// //solution 1 use hashmap
// use std::collections::HashMap;
// use std::cmp::max;

// fn length_of_longest_substring(s: String) -> i32 {
//     let mut m = HashMap::new();
//     let mut ans = 0;
//     let mut before = -1;
//     let mut current = 0;
//     for c in s.chars() {
//         if let Some(last) = m.insert(c, current) {
//             before = max(before, last);
//         }
//         ans = max(ans, current - before);
//         current += 1;
//     }
//     ans
// }

// solution 2: dont use hashmap
fn length_of_longest_substring(s: String) -> i32 {
    let mut max_len: usize = 0;
        
    // [1] longest substring is the one with the largest
    //     difference of positions of repeated characters;
    //     thus, we should create a storage for such positions
    let mut pos: [usize;128] = [0;128];

    // [2] while iterating through the string (i.e., moving
    //     the end of the sliding window), we should also
    //     update the start of the window
    let mut start: usize = 0;

    for (end, ch) in s.chars().enumerate()
    {
        // [3] get the position for the start of sliding window
        //     with no other occurences of 'ch' in it
        start = start.max(pos[ch as usize]);
        
        // [4] update maximum length 
        max_len = max_len.max(end-start+1);
        
        // [5] set the position to be used in [3] on next iterations
        pos[ch as usize] = end + 1;
    }
            
    return max_len as i32;
}



fn main() {
    // let a = String::from("taaaa");
    // let mut b: Vec<u8> = a.as_bytes().into();
    // b.dedup();
    // println!("{:?}",b);

    let s = String::from("abcdabcbb");
    println!("{}",length_of_longest_substring(s));
}