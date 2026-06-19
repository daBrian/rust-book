use std::cmp::Ordering::{Equal, Less};

fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    use_greater_or_equal(5, 2)
}

fn use_greater_or_equal(first: i32, second: i32) {
    let comparator = first.cmp(&second);
    if comparator== Equal {
        println!("first and second are equal");
    } else if comparator== Less {
        println!("first is less than second");
    } else {
        println!("first is more than second");
    }
}

