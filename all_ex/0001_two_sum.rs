fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (key, value) in nums.iter().enumerate() {
        if nums.contains(&(target-value)){
            let key2 = nums.iter().position(|&x| x==(target-value)).unwrap();
            if key2 != key {
                return vec![key as i32, key2 as i32];
            }
        } 
    }
    return vec![];
}




fn main() {
    // let nums = vec![2,7,11,15];
    // let target = 9;

    let nums = vec![3,2,4];
    let target = 6;
    let a = two_sum(nums, target);
    println!("{:?}",a);
}
