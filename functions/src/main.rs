fn main() {
    println!("Hello, world!");

    another_function(5);

    let y = {
        let x = 3;
        // No semi-colon makes this block an expression instead of a statement.
        x + 1
    };

    println!("The value of y is {}", y);

    let z = five();

    println!("The value of z id {}", z);
}

fn another_function(x: i32) {
    println!("The value of x if {}", x)
}

// Returns 5
fn five() -> i32 {
    5
}
