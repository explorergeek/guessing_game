use std::io;
use std::cmp::Ordering;


pub fn guess(num: i32, round: i32) -> bool {
    let mut tries = 1;
    let mut flag = false;
    println!("{num}");
    loop {
        if tries <= round {
            let mut guess = String::new();

            io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

            let guess: i32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            match guess.cmp(&num) {
                Ordering::Less => {
                    println!("Too small!");
                    tries += 1;
                }
                Ordering::Greater => {
                    println!("Too big!");
                    tries += 1;
                }
                Ordering::Equal => {
                    println!("You guessed right!");
                    tries = 10;
                    flag = true;
                }
            };
        }
        else {
            break;
        }
    }
        return flag
}

