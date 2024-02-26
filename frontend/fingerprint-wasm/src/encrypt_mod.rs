use std::char;

fn char_shift(c: char, shift: u8, base: u8) -> char {
    let c_u8 = c as u8;
    (((c_u8 - base + shift) % 26) + base) as char
}

pub fn encrypt_vigenere(data: &str, secret: &str) -> String {
    let mut cleaned_secret = secret.chars().filter(|c| c.is_ascii_alphabetic()).collect::<String>();
    if cleaned_secret.is_empty() {
        cleaned_secret = "XGGHRDFGJKWKLGNG".to_string();
    }else{
        cleaned_secret = cleaned_secret.to_uppercase();
    }
    let mut output = String::new();
    let secret_bytes = cleaned_secret.as_bytes();

    for (i, c) in data.chars().enumerate() {
        let secret_char = secret_bytes[i % cleaned_secret.len()] - b'A';
        let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };

        if c.is_ascii_alphabetic() {
            output.push(char_shift(c, secret_char, base));
        } else {
            output.push(c);
        }
    }

    output
}

pub fn decrypt_vigenere(encrypted_data: &str, secret: &str) -> String {
    let mut cleaned_secret = secret.chars().filter(|c| c.is_ascii_alphabetic()).collect::<String>();
    if cleaned_secret.is_empty() {
        cleaned_secret = "XGGHRDFGJKWKLGNG".to_string();
    }else{
        cleaned_secret = cleaned_secret.to_uppercase();
    }
    let mut output = String::new();
    let secret_bytes = cleaned_secret.as_bytes();

    for (i, c) in encrypted_data.chars().enumerate() {
        let secret_char = 26 - (secret_bytes[i % cleaned_secret.len()] - b'A');
        let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };

        if c.is_ascii_alphabetic() {
            output.push(char_shift(c, secret_char, base));
        } else {
            output.push(c);
        }
    }

    output
}

fn calculate_checksum(data: &str) -> u32 {
    data.chars().map(|c| c as u32).sum()
}

pub fn encrypt(data: &str, secret: &str) -> String {
    let encrypted_data = encrypt_vigenere(data, secret);
    let checksum = calculate_checksum(&encrypted_data);
    format!("{}{:08X}", encrypted_data, checksum)
}

pub fn decrypt(encrypted_data_with_checksum: &str, secret: &str) -> Result<String, String> {
    if encrypted_data_with_checksum.len() < 9 {
        return Err("not enough data".to_string());
    }

    let encrypted_data = &encrypted_data_with_checksum[..encrypted_data_with_checksum.len() - 8];
    let checksum_str = &encrypted_data_with_checksum[encrypted_data_with_checksum.len() - 8..];
    let checksum = u32::from_str_radix(checksum_str, 16).map_err(|_| "invalid".to_string())?;

    let decrypted_data = decrypt_vigenere(encrypted_data, secret);

    let calculated_checksum = calculate_checksum(encrypted_data);
    if calculated_checksum == checksum {
        Ok(decrypted_data)
    } else {
        Err("mismatch".to_string())
    }
}