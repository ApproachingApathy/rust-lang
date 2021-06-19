fn main() {
    println!("Hello, world!");
    // Defining a struct.
    // Initialize the struct
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someone"),
        active: true,
        sign_in_count: 1,
    };
    println!("{}", user1.email);
    // Initialize a mutable struct
    // The entire struct is mutable. You cannot define mutability for fields.
    let mut user2 = User {
        email: String::from("noone@example.com"),
        username: String::from("noone"),
        active: false,
        sign_in_count: 0,
    };
    user2.username = String::from("who");
    println!("{}", user2.username);

    // We can create a new instance from a struct from another instance.
    // Think "spread" operator in js.
    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("another"),
        ..user1
    };

    println!(
        "User \"{}\" is {}",
        user3.username,
        if user3.active { "active" } else { "inactive" }
    );

    // You can make a tuple struct that has only values.
    struct Color(i32, i32, i32);

    let black = Color(0, 0, 0);

    println!("{}, {}, {}", black.0, black.1, black.2);

    let rec = Rectangle {
        height: 10,
        width: 5,
    };

    println!("The area of the rectangle is {}.", rec.area());

    let square = Rectangle::square(5);

    println!("The rectangle is {width}x{width}", width = square.width);

    println!("The rectangle has an area of {}", square.area());
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Rectangle {
    width: u32,
    height: u32,
}

// We can create methods on a struct
impl Rectangle {
    // Create a method
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Create an associated function (static method)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
