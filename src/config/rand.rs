use rand::{distributions::Alphanumeric, Rng};

pub fn random_string(num: usize) -> String {
    let random_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(num) // Adjust length as needed
        .map(char::from)
        .collect();

    random_string
}
