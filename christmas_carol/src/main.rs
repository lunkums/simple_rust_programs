fn main() {
    for day in 1..=12 {
        match day_to_string(day) {
            Some(curr) => {
                println!("On the {curr} day of Christmas my true love sent to me");
            }
            None => {
                break;
            }
        }

        if day == 1 {
            println!("A partridge in a pear tree");
        } else {
            for day in (2..=day).rev() {
                if day == 12 {
                    print!("Twelve drummers drumming");
                } else if day == 11 {
                    print!("Eleven pipers piping");
                } else if day == 10 {
                    print!("Ten lords a-leaping");
                } else if day == 9 {
                    print!("Nine ladies dancing");
                } else if day == 8 {
                    print!("Eight maids a-milking");
                } else if day == 7 {
                    print!("Seven swans a-swimming");
                } else if day == 6 {
                    print!("Six geese a-laying");
                } else if day == 5 {
                    print!("Five gold rings");
                } else if day == 4 {
                    print!("Four calling birds");
                } else if day == 3 {
                    print!("Three French hens");
                } else if day == 2 {
                    print!("Two turtle doves");
                }
                println!(",")
            }
            println!("And a partridge in a pear tree.");
        }
        println!();
    }
}

fn day_to_string(day: u8) -> Option<String> {
    match day {
        0 => None,
        1 => Some(String::from("first")),
        2 => Some(String::from("second")),
        3 => Some(String::from("third")),
        4 => Some(String::from("fourth")),
        5 => Some(String::from("fifth")),
        6 => Some(String::from("sixth")),
        7 => Some(String::from("seventh")),
        8 => Some(String::from("eighth")),
        9 => Some(String::from("ninth")),
        10 => Some(String::from("tenth")),
        11 => Some(String::from("eleventh")),
        12 => Some(String::from("twelfth")),
        (13_u8..=u8::MAX) => None,
    }
}
