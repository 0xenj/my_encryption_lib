// Import the function for removing accents from characters.
use crate::utils::replace_accented_char;

/// Shifts a given character by a specified number of positions in the alphabet,
/// either forward (encrypt) or backward (decrypt).
///
/// # Arguments
/// * `letter` - The character to be shifted.
/// * `shift` - The number of positions to shift the character.
/// * `encrypt` - A boolean indicating whether to encrypt (true) or decrypt (false).
///
/// # Returns
/// Returns the shifted character. If the input character is not alphabetic,
/// it returns the character as is.
fn shift_char(letter: char, shift: i32, encrypt: bool) -> char {
    if !letter.is_alphabetic() {
        return letter;
    }

    let base = if letter.is_uppercase() { 'A' } else { 'a' } as u8;
    let letter_pos = letter as u8 - base;
    let shift = if encrypt { shift } else { -shift };
    let new_pos = ((letter_pos as i32 + shift) % 26 + 26) % 26;
    (base + new_pos as u8) as char
}

/// Converts a letter to its shift value based on its position in the alphabet.
///
/// # Arguments
/// * `letter` - The letter to convert to a shift value.
///
/// # Returns
/// Returns the shift value as an integer.
fn char_to_shift(letter: char) -> i32 {
    let base = if letter.is_uppercase() { 'A' } else { 'a' } as u8;
    (letter as u8 - base) as i32
}

/// Encrypts or decrypts a line of text based on a password. Each letter in the password
/// determines the shift value for the corresponding letter in the text.
///
/// # Arguments
/// * `line` - The text to be encrypted or decrypted.
/// * `password` - The password used for calculating the shift value.
/// * `encrypt` - A boolean indicating whether to encrypt (true) or decrypt (false).
///
/// # Returns
/// Returns a `Result` with either the encrypted/decrypted string or an error message
/// for invalid input conditions.
pub fn mel_mdp(line: &str, password: &str, encrypt: bool) -> Result<String, &'static str> {
    if line.is_empty() {
        return Err("Text must not be empty");
    }

    if password.is_empty() || password.chars().any(|c| !c.is_alphabetic()) {
        return Err("Password must not be empty and must only contain alphabetic characters");
    }

    let mut converted = String::new();
    let mut pass_index = 0;
    let pass_len = password.len();

    for ch in line.chars() {
        let ch_no_accent = replace_accented_char(ch);
        if ch_no_accent.is_alphabetic() {
            let pass_char: char = password.chars().nth(pass_index % pass_len).unwrap();
            let shift = char_to_shift(pass_char);
            converted.push(shift_char(ch_no_accent, shift, encrypt));
            pass_index += 1;
        } else {
            converted.push(ch); // Keep non-alphabetic characters unchanged
        }
    }

    Ok(converted)
}
