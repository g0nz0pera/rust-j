fn main() {
    //get_sum(0, 1);
    assert_eq!(get_sum(0, 1), 1);
    assert_eq!(get_sum(1, 2), 3);
    assert_eq!(get_sum(5, -1), 14);
    assert_eq!(get_sum(505, 4), 127759);
}

fn get_sum(a: i64, b: i64) -> i64 {
    let mut res: i64 = 0;
    if a == b {
        return a;
    }
    if a<b  {
        for number in a..b+1 {
            res = res + number;
        };
    }
    if a>b{
        for number in b..a+1 {
            res = res + number;
        };
    };
    return res;

}



/* Other way of doing this
   fn get_sum(a: i64, b: i64) -> i64 {
       (a.min(b)..=a.max(b)).sum()
   }

   fn get_sum(a: i64, b: i64) -> i64 {
    (a + b) * ((a - b).abs() + 1) / 2
   }

   fn get_sum(a: i64, b: i64) -> i64 {
     match a < b {
         true => (a..=b).sum(),
         false => (b..=a).sum(),
     }
   }
*/
