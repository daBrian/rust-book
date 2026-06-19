fn main() {
    variables();
    tuples();
}

fn variables() {
    // 'mut' makes x mutable
    let mut m = 5;
    println!("The value of m is: {m}");
    m = 6;
    println!("The value of m is: {m}");

    //shadowing
    let x = 5;
    println!("The value of x is: {x}");

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of new x is: {x}");

    const Z: i32 = 1;
    println!("The value of Z is: {Z}");

    // shadowing const is not allowed:
    // const Z: i32 = 2;
    // println!("The value of Z is: {Z}");
}

fn tuples() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
    let second = tup.1;
    println!("The value of the second element is: {second}");
}
