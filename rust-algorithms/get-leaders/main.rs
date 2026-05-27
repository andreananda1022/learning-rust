fn main() {
    let input1: [i32; 7] = [9, 21, 8, 13, 3, 7, 5];
    let input2: [i32; 7] = [6, 14, 17, 3, 12, 4, 7];

    println!("{:?}", get_leaders(&input1));    // Output: [21, 13, 7, 5]
    println!("{:?}", get_leaders(&input2));    // Output: [17, 12, 7]
}

fn get_leaders(input: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();

    let mut max_from_right = input[input.len() - 1];
    result.push(max_from_right);

    for i in (0..input.len() - 1).rev() {
        if input[i] > max_from_right {
            max_from_right = input[i];
            result.push(max_from_right);
        }
    }

    result.reverse();
    result
}

// Rusty Logic
// Sluggish
// Profligate
// fn get_leaders(input: &[i32]) -> Vec<i32> {
//     let mut result = Vec::new();
//     for i in 0..input.len() {
//         let left_number = input[i];
//         let end_number = input[input.len() - 1];
//         for j in (i + 1)..input.len() {
//             let right_number = input[j];
//             if left_number < right_number || left_number == end_number {
//                 result.push(right_number);
//             }
//         }
//     }
// 
//     result
// }
