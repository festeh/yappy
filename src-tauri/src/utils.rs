use rand::{distributions::Alphanumeric, Rng};
use std::iter;

pub fn generate_random_id() -> String {
    let mut rng = rand::thread_rng();

    iter::repeat(())
        .map(|()| rng.sample(Alphanumeric).to_ascii_uppercase() as char)
        .filter(|c| c.is_ascii_alphanumeric() && (c.is_ascii_uppercase() || c.is_ascii_digit()))
        .take(8)
        .collect()
}
