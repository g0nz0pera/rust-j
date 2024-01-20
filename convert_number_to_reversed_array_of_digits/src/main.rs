fn main() {
    assert_eq!(digitize(348597), vec![7,9,5,8,4,3]);
    assert_eq!(digitize(35231), vec![1,3,2,5,3]);
    assert_eq!(digitize(0), vec![0]);

    assert_eq!(digitize_lamb(348597), vec![7,9,5,8,4,3]);
    assert_eq!(digitize_lamb(35231), vec![1,3,2,5,3]);
    assert_eq!(digitize_lamb(0), vec![0]);
}

fn digitize(n: u64) -> Vec<u8> {
    let mut stack = Vec::new();
    for c in n.to_string().chars() {
        stack.push(c.to_digit(10).unwrap());
    }
    return stack.iter().rev().map(|c| *c as u8).collect::<Vec<_>>();
}

fn digitize_lamb(n: u64) -> Vec<u8> {
    n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .rev()
        .collect::<Vec<u8>>()
}