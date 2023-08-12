use std::io;

fn main() {
    loop {
        let n = get_num();
        if n == 0 {
            break;
        }
        let ans = fibonacci(n);
        println!("    {ans}");
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
                println!("'{input}' is not a number. Try again.");
                continue;
            }
        };
    };

    num
}

fn fibonacci(n: u32) -> u64 {
    let mut i: u32 = n;
    let mut ans: u64 = 0;
    let mut last: u64 = 1;

    while i > 0 {
        let temp = ans;
        ans += last;
        last = temp;
        i -= 1;
    }

    ans
}
