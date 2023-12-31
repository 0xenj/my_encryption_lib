// Import the function for removing accents from characters.
use crate::utils::replace_accented_char;

/// Shifts a given character by a specified number of positions in the alphabet.
///
/// # Arguments
/// * `letter` - The character to be shifted.
/// * `shift` - The number of positions to shift the character.
///
/// # Returns
/// Returns the shifted character. If the input character is not alphabetic,
/// it returns the character as is.
fn shift_char(letter: char, shift: i32) -> char {
    // Return the character unchanged if it's not an alphabetic character.
    if !letter.is_alphabetic() {
        return letter;
    }

    // Determine the ASCII values for the start and end of the alphabet, 
    // depending on whether the letter is uppercase or lowercase.
    let (start, end) = if letter.is_uppercase() { ('A' as u8, 'Z' as u8) } else { ('a' as u8, 'z' as u8) };
    let first_ascii = start as i32;
    let last_ascii = end as i32;

    // Perform the shift operation, adjusting for wrapping around the alphabet.
    let mut shifted_ascii = letter as i32 + shift;
    while shifted_ascii > last_ascii {
        shifted_ascii -= 26;
    }
    while shifted_ascii < first_ascii {
        shifted_ascii += 26;
    }

    // Convert the shifted ASCII value back to a character.
    std::char::from_u32(shifted_ascii as u32).unwrap()
}

/// Encrypts or decrypts a given line of text using a specified shift value.
///
/// # Arguments
/// * `line` - The text to be encrypted or decrypted.
/// * `initial_shift` - The initial shift value for the encryption or decryption.
/// * `encrypt` - A boolean flag that determines whether to encrypt or decrypt.
///
/// # Returns
/// Returns a `Result` containing either the transformed text or an error message
/// if the input text is empty.
pub(crate) fn mel(line: &str, initial_shift: i32, encrypt: bool) -> Result<String, &'static str> {
    // Return an error if the input text is empty.
    if line.is_empty() {
        return Err("Text must not be empty");
    }

    let mut converted = String::new();
    // Determine the shift direction based on the encrypt flag.
    let shift = if encrypt { initial_shift } else { -initial_shift };

    // Iterate over each character in the input line.
    for letter in line.chars() {
        // Replace accented characters with their non-accented equivalents.
        let letter_no_accent = replace_accented_char(letter);
        // Apply the shift to alphabetic characters and add them to the result string.
        if letter_no_accent.is_alphabetic() {
            converted.push(shift_char(letter_no_accent, shift));
        } else {
            // Keep non-alphabetic characters as they are.
            converted.push(letter);
        }
    }

    // Return the transformed text.
    Ok(converted)
}
