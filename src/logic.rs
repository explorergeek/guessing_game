use std::io;
use std::cmp::Ordering;

// Parameters:
// num: An integer value representing the number to be guessed
// round: An integer value representing the number of rounds the user has to guess the number.

// Return Value: 
// A boolean value representing whether the user has guessed the correct number or not.
// true if the user guessed correctly within the specified rounds, false otherwise.

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

