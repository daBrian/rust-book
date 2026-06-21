fn main() {
    let s = &String::from("hello world");
    let _s2 = "some text";
    let part =first_word(s);
    println!("{part}")
}

#[allow(dead_code)]
fn c4_1_introduction() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
    //not valid
    // println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
