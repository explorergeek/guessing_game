use rand::Rng;

pub fn gen_low_num() -> i32 {
    let low_num = rand::thread_rng().gen_range(1..=10);
    low_num
}
pub fn gen_mid_num() -> i32 {
    let mid_num = rand::thread_rng().gen_range(11..=20);
    mid_num
}

pub fn gen_high_num() -> i32 {
    let high_num = rand::thread_rng().gen_range(21..=30);
    high_num
}