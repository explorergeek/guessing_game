mod secret_numbers;
mod logic;

fn main() {
    println!("GUESS THE NUMBER");
    
    //Gets randomly generated numbers from secret_numbers file
    let low_sc_num = secret_numbers::gen_low_num();
    let mid_sc_num = secret_numbers::gen_mid_num();
    let high_sc_num = secret_numbers::gen_high_num();

    //How many tries the user gets each round
    const ROUND_ONE_TRIES: i32 = 3;
    const ROUND_TWO_TRIES: i32 = 2;
    const ROUND_THREE_TRIES: i32 = 1;

    let mut flag = false;

    //First attempt
    println!("Please input your guess for the lower number. You will only have 3 attempts to pass this level. HINT: Its 1-10.");
    flag = logic::guess(low_sc_num, ROUND_ONE_TRIES);

    //Second attempt
    if flag {
        println!("Please input your guess for the middle number. You will only have 2 attempts to pass this level. HINT: Its 11-20.");
        flag = logic::guess(mid_sc_num, ROUND_TWO_TRIES);
    }

    //Third attempt
    if flag {
        println!("Please input your guess for the high number. You will only have 1 attempt to pass this level. HINT: Its 21-30.");
        flag = logic::guess(high_sc_num, ROUND_THREE_TRIES);
    }
}
