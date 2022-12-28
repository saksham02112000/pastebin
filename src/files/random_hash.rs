use rand::{thread_rng, Rng, distributions::Alphanumeric};

pub async fn generate_random_hash() -> String{
    let random_number: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    return random_number.into();
}
