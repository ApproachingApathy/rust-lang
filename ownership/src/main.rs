fn main() {
    // let s = "hello"
    let mut s = String::from("hello");

    s.push_str(", world");

    println!("{}", s);

    // Passes a value
    let x = 5;
    let y = x;

    println!("{}", y);

    // Move a reference, s1 is no longer valid.
    // let s1 = String::from("hello");
    // let s2 = s1;

    // // println!("{}", s1);

    // To copy
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}.", s1, s2);

    let h = String::from("hello");

    takes_ownership(h); // h moves out of scope.

    let z = 5;

    makes_copy(z);

    println!("{}", z);

    let s4 = String::from("hello");

    let len = calculate_length(&s4); // &s4 is a reference.

    println!("The length of {} is {}.", s4, len);

    // Mutable references
    let mut s5 = String::from("hello");

    change(&mut s5);

    println!("{}", s5);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
