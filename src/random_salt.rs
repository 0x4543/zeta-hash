use rand::Rng;
use crate::constants::SALT_CHARSET;

pub fn generate_salt(len: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..len)
        .map(|_| {
            let idx = rng.gen_range(0..SALT_CHARSET.len());
            SALT_CHARSET[idx] as char
        })
        .collect()
}