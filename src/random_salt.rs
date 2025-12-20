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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_salt_length() {
        let len = 32;
        let salt = generate_salt(len);
        assert_eq!(salt.len(), len);
    }

    #[test]
    fn test_salt_charset() {
        let salt = generate_salt(100);
        for c in salt.chars() {
            assert!(crate::constants::SALT_CHARSET.contains(&(c as u8)));
        }
    }
}