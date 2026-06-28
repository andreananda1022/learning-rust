fn add_two_numbers(l1: &Vec<u8>, l2: &Vec<u8>) -> Vec<u8> {
    let mut result = Vec::new();

    let mut carry = 0;
    let mut i = 0;

    while i < l1.len() || i < l2.len() || carry > 0 {
        let x = if i < l1.len() {
            l1[i]
        } else {
            0
        };

        let y = if i < l2.len() {
            l2[i]
        } else {
            0
        };

        let sum = x + y + carry;

        result.push(sum % 10);
        carry = sum / 10;

        i += 1;
    }

    result
}

fn main() {
    let l1 = vec![2, 4, 7];
    let l2 = vec![5, 6, 4];

    let result = add_two_numbers(&l1, &l2);

    println!("{:?}", result);
}
