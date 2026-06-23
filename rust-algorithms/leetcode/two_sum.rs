use std::collections::HashMap;

fn main() {
    let nums = vec![2, 2, 5, 4, 3];
    let target = 9;

    let result = two_sum(nums, target);
    println!("{:?}", result);
}

// Pendekatan Menggunakan HashMap
// Time Complexity: O(n)
fn two_sum(nums: Vec<i8>, target: i8) -> Vec<i8> {
    let mut map = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&j) = map.get(&complement) {
            return vec![j as i8, i as i8];
        }
        map.insert(num, i);
    }
    vec![]
}
