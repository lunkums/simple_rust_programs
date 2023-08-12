use std::io;

fn main() {
    loop {
        let n = get_num();
        if n == 0 {
            break;
        }
        match fibonacci(n) {
            Some(ans) => println!("    {ans}"),
            None => println!("    The answer was too big. Try a smaller number."),
        }
        println!();
    }
    println!("Goodbye!");
}

fn get_num() -> u32 {
    println!("Enter a positive whole number, or 0 to exit:");

    let mut input;

    let num = loop {
        input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        let input = input.trim();
        match input.parse() {
            Ok(num) => {
                break num;
            }
            Err(_) => {
                println!("'{input}' is either not a number or is too large. Try again.");
                continue;
            }
        };
    };

    num
}

fn fibonacci(n: u32) -> Option<u64> {
    let mut ans: u64 = 0;
    let mut last: u64 = 1;

    for _ in 0..n {
        let temp = ans;
        ans = match ans.checked_add(last) {
            Some(ans) => ans,
            None => {
                return None;
            }
        };
        last = temp;
    }

    Some(ans)
}
