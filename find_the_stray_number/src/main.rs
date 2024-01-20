fn main() {
    println!("Running 1");

    let actual = stray_third(&[1, 1, 1, 1, 1, 1, 2]);
    let expected = 2;
    assert!(actual == expected,
            "\nExpected {expected} but got {actual}");
    println!("Running 2");

    let actual2 = stray_third(&[2, 3, 2, 2, 2]);
    let expected2 = 2;
    assert!(actual2 == expected2,
            "\nExpected {expected2} but got {actual2}");

    println!("Running 3");
    let actual3 = stray_third(&[3, 2, 2, 2, 2]);
    let expected3 = 3;
    assert!(actual3 == expected3,
            "\nExpected {expected3} but got {actual3}");
}



fn stray(arr: &[u32]) -> u32 {

    let i = 0;
    let mut j = 1;
    let mut result = 0;
    let mut found = false;
    while j <= arr.len()-1 && found == false {
        if j == arr.len()-1{
            result = *arr.get(j).unwrap();
            found = true;
        }else{
            if *arr.get(i).unwrap() == *arr.get(j).unwrap() {
                j += 1
            } else if *arr.get(i).unwrap() != *arr.get(j).unwrap() {
                if *arr.get(i).unwrap() == *arr.get(j+1).unwrap() {
                    result = *arr.get(j).unwrap();
                    found = true;
                }else if *arr.get(j).unwrap() == *arr.get(j+1).unwrap(){
                    result = *arr.get(i).unwrap();
                    found = true;
                }

            }
        }



    }
    return result;
}

fn stray_second(arr: &[u32]) -> u32 {
    arr.iter().fold(0, |acc, el| acc ^ el)
}

fn stray_third(arr: &[u32]) -> u32 {

    for item in arr {
        if arr.iter().filter(|&x| x == item).count() == 1 {
            return *item;
        }
    }
    unreachable!();
}


// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::stray;

    fn dotest(arr: &[u32], expected: u32) {
        let actual = stray(arr);
        assert!(actual == expected,
                "With arr = {arr:?}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests() {
        dotest(&[1, 1, 1, 1, 1, 1, 2], 2);
        dotest(&[2, 3, 2, 2, 2], 3);
        dotest(&[3, 2, 2, 2, 2], 3);
    }
}
