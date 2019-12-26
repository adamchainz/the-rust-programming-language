fn main() {
    let items = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];
    for day in 1..13 {
        println!(
            "🎄 On the {}{} day of Christmas my true love sent to me:",
            day,
            ordinal(day)
        );
        for item in (0..day).rev() {
            println!("  ➡️  {}", items[item]);
        }
        println!("")
    }
}

fn ordinal(n: usize) -> String {
    if n == 0 || n == 1 {
        "st"
    } else if n == 2 {
        "nd"
    } else if n == 3 {
        "rd"
    } else {
        "th"
    }
    .to_string()
}
