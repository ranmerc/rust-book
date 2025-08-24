fn main() {
    let days_in_words = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let items = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for day in 1..13 {
        let current_day = days_in_words[day - 1];
        println!("On the {current_day} day of Christmas, my true love sent to me");

        for line in (0..day).rev() {
            let current_line = items[line];

            print!("{current_line}");

            if line == 1 {
                print!(" and")
            }

            println!("");
        }

        println!("");
    }
}
