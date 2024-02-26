
pub fn encrypt(data: &str, secret: &str) -> String {
    let mut output = String::new();
    for (i, c) in data.chars().enumerate() {
        let secret_char = secret.chars().nth(i % secret.len()).unwrap();
        let xored = c as u8 ^ secret_char as u8;
        output.push(xored as char);
    }
    output
}

pub fn decrypt(encrypted_data: &str, secret: &str) -> String {
    encrypt(encrypted_data, secret)
}