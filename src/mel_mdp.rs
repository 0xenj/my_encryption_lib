use crate::utils::replace_accented_char;

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

fn char_to_shift(letter: char) -> i32 {
    let base = if letter.is_uppercase() { 'A' } else { 'a' } as u8;
    (letter as u8 - base) as i32
}

pub fn mel_mdp(line: &str, password: &str, encrypt: bool) -> String {
    let mut converted = String::new();
    let mut pass_index = 0;
    let pass_len = password.len();

    for ch in line.chars() {
        let ch_no_accent = replace_accented_char(ch);
        if ch_no_accent.is_alphabetic() {
            let pass_char = password.chars().nth(pass_index % pass_len).unwrap();
            let shift = char_to_shift(pass_char);
            converted.push(shift_char(ch_no_accent, shift, encrypt));
            pass_index += 1;
        } else {
            converted.push(ch_no_accent);
        }
    }

    converted
}
