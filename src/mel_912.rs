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
    if !letter.is_alphabetic() {
      return letter;
    }

    // Determine the ASCII range for the letter based on its case.
    let (start, end) = if letter.is_uppercase() { ('A', 'Z') } else { ('a', 'z') };
    let first_ascii = start as i32;
    let last_ascii = end as i32;
    let alphabet_length = 26;

    // Perform the shift, adjusting for alphabet wrapping.
    let mut shifted_ascii = (letter as i32) + shift;
    while shifted_ascii > last_ascii {
      shifted_ascii -= alphabet_length;
    }
    while shifted_ascii < first_ascii {
      shifted_ascii += alphabet_length;
    }

    std::char::from_u32(shifted_ascii as u32).unwrap_or(letter)
}

/// Updates the shift value based on the current shift and whether we are encrypting or decrypting.
///
/// # Arguments
/// * `shift` - The current shift value.
/// * `encrypt` - A boolean flag indicating encryption (true) or decryption (false).
///
/// # Returns
/// Returns the updated shift value.
fn update_shift(shift: i32, encrypt: bool) -> i32 {
    let mut new_shift = shift + if encrypt { -1 } else { 1 };
    if new_shift == 2 || new_shift == -2 {
        new_shift += if encrypt { 9 } else { -9 };
    }
    new_shift
}

/// Encrypts or decrypts a given line of text using a variable shift cipher.
///
/// # Arguments
/// * `line` - The text to be encrypted or decrypted.
/// * `encrypt` - A boolean flag to specify encryption (true) or decryption (false).
///
/// # Returns
/// Returns a `Result` with either the encrypted/decrypted string or an error if the input is empty.
pub fn mel_912(line: &str, encrypt: bool) -> Result<String, &'static str> {
    if line.is_empty() {
      return Err("Text must not be empty");
    }

    let mut converted = String::new();
    let mut shift = if encrypt { 9 } else { -9 };

    // Iterate over each character in the input line.
    for letter in line.chars() {
      // Replace accented characters with their non-accented equivalents.
      let letter_no_accent = replace_accented_char(letter);
      if letter_no_accent.is_alphabetic() {
          // Apply the shifting cipher and update the shift value.
          converted.push(shift_char(letter_no_accent, shift));
          shift = update_shift(shift, encrypt);
      } else {
          // Keep non-alphabetic characters as they are.
          converted.push(letter);
      }
  }

    Ok(converted)
}
