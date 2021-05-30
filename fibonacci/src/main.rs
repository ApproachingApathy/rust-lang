use std::io;

fn main() {
    println!("===== Fibonacci Generator =====");

    let mut target = String::new();

    let target: u32 = loop {
        println!("Please input the target number.");

        io::stdin()
            .read_line(&mut target)
            .expect("Failed to read line");
        match target.trim().parse::<u32>() {
            Ok(num) => break num,
            Err(_) => continue,
        }
    };

    println!("Calculating fibonacci to position {}", target);

    let mut last: u32 = 0;

    for position in (0..target) {}
}
