const DAYS: usize = 12;

const GIFTS: [&str; DAYS] = [
    "partridge in a pear tree",
    "turtle doves",
    "french hens",
    "calling birds",
    "golden rings",
    "geese a' laying",
    "swans a' swimming",
    "maids a' milking",
    "ladies dancing",
    "lords a' leaping",
    "pipers piping",
    "drummers drumming",
];

const DAY_NAMES: [&str; DAYS] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

const NUMERICS: [&str; DAYS] = [
    "a", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven", "twelve",
];

fn main() {
    println!("");
    println!("== The Twelve Days of Christmas ==");
    println!("");

    for verse in 0..DAYS {
        println!(
            "🎶 On the {} day of christmas, my true love sent to me: 🎶",
            DAY_NAMES[verse]
        );

        for day in (0..verse + 1).rev() {
            println!(
                "🎶 {}{} {}{} 🎶",
                if verse != 0 && day == 0 {"and "} else {""},
                NUMERICS[day],
                GIFTS[day],
                if day == 0 { "." } else { "," }
            );
        }

        println!("");
    }
}
