mod secret_numbers;
mod logic;

fn main() {
    println!("GUESS THE NUMBER");
    println!("For the first round, you will have 3 guesses to guess the correct number. If you guess right, you will move on to the second round.");
    println!("For the second round, you will have 2 guesses to guess the correct number. If you guess right, you will move on to the last round.");
    println!("For the final round, you will have 1 guess to guess the correct number and win the game!");
    
    //Gets randomly generated numbers from secret_numbers file
    let low_sc_num = secret_numbers::gen_low_num();
    let mid_sc_num = secret_numbers::gen_mid_num();
    let high_sc_num = secret_numbers::gen_high_num();
    const ROUND_ONE_TRIES: i32 = 3;
    const ROUND_TWO_TRIES: i32 = 2;
    const ROUND_THREE_TRIES: i32 = 1;
    let mut tries = 0;

    //First attempt
    println!("Please input your guess for the low number. You will only have 3 attempts to pass this level. HINT: Its 1-10.");
    tries = logic::guess(low_sc_num, ROUND_ONE_TRIES);

    //Second attempt
    if tries == 10 {
        println!("Please input your guess for the mid number. You will only have 2 attempts to pass this level. HINT: Its 11-20.");
        tries = logic::guess(mid_sc_num, ROUND_TWO_TRIES);
    }

    //Third attempt
    if tries == 10 {
        println!("Please input your guess for the high number. You will only have 1 attempt to pass this level. HINT: Its 21-30.");
        tries = logic::guess(high_sc_num, ROUND_THREE_TRIES);
    }
}
