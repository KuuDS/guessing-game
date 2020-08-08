use rand::Rng;
use std::cmp::Ordering;
use std::io;
// import io library in std(standard) library

fn main() {
    // declare a main method which means the entrance for this program.
    println!("Guess the number!");
    // println! is a macro

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");
        // declare a mutable variable.
        let mut guess = String::new();

        io::stdin()
            // the & before argument, indicates that it's a reference
            .read_line(&mut guess)
            // read_line returns io::Result, it a submodule of std::Result
            .expect("Fail to read line");
        // 必须判断Result的类型，如果是Err则会执行expect

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);
        // 使用placeholders打印字符串

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
