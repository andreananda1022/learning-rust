fn main() {
    let nums: [i8; 5] = [2, 2, 5, 3, 3];
    let target = 9;

    let result = two_sum(nums, target);
    println!("{:?}", result);
}

// Brute Force
// Time Complexity: O(n^2)
fn two_sum(nums: [i8; 5], target: i8) -> [usize; 2] {
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            // Cara sebelumnya seperti ini: if nums[i] + nums[j] == target
            // Katanya lebih dekat dengan algoritma yang digunakan dalam HashMap
            if nums[i] == target - nums[j] {
                return [i, j]
            }
        }
    }

    [0, 0]
}
