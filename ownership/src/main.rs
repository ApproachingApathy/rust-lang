fn main() {
    // let s = "hello"
    let mut s = String::from("hello");

    s.push_str(", world");

    println!("{}", s);

    // These values are copied.
    // Both stacked memory values are valid.
    // Passes a value
    let x = 5;
    let y = x;

    println!("{}", y);

    // This moves the reference, rather than copying it. s1 is no longer valid.
    // This avoids a double free error that would be caused when rust attempts to drop the values from memory.
    // let s1 = String::from("hello");
    // let s2 = s1;

    // // println!("{}", s1);

    // In order to copy a heaped memory value. We need to explicitly do so.
    // Both values
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}.", s1, s2);

    let h = String::from("hello");

    /* Function params behave like variables:
        - stack memory is copied and remains in scope
        - heap memory is moved and falls out of scope.
    Because `h` is a mutable string, its memory is on the heap and `h` falls out of scope. */
    takes_ownership(h);

    let z = 5;

    /* `z` is an integer and is stored on the stack. `z` does not fall out of scope. */
    makes_copy(z);

    println!("{}", z);

    /* We can avoid our a heap variable from going out of scope by using references.
    Let's initialize `s4` a mutable string (heaped) and pass it to a function. */
    let s4 = String::from("hello");

    /* By passing a reference (`&s4`) to the function we can keep `s4` in scope. */
    let len = calculate_length(&s4); // &s4 is a reference.

    println!("The length of {} is {}.", s4, len);

    /* We can pass mutable */
    let mut s5 = String::from("hello");

    change(&mut s5);

    println!("{}", s5);

    let honey = "Hi, Honey!";

    let s5 = first_word(honey);

    println!("{}", s5);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

/**
 * Returns the length of a string.
 */
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &str) -> &str {
    // First we convert the string to a byte array.
    let bytes = s.as_bytes();

    // We'll use the array's `.iter` method to iterate over the byte array.
    // `enumerate` wraps `iter` an returns a tuple `(index, item)`.
    for (i, &item) in bytes.iter().enumerate() {
        // Here we find a space by comparing `item` to a "byte literal" (`b' '`).
        if item == b' ' {
            // When find it we return the index.
            return &s[0..i];
        }
    }

    // If the for completes without finding a space. Return the length of the String.
    &s[..]
}
