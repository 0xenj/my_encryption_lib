use crate::utils::replace_accented_char;

fn shift_char(letter: char, shift: i32) -> char {
    let (start, end) = if letter.is_uppercase() { ('A' as u8, 'Z' as u8) } else { ('a' as u8, 'z' as u8) };
    
    if letter.is_alphabetic() {
        let first_ascii = start as i32;
        let last_ascii = end as i32;
        let mut shifted_ascii = letter as i32 + shift;

        while shifted_ascii > last_ascii {
            shifted_ascii -= 26;
        }
        while shifted_ascii < first_ascii {
            shifted_ascii += 26;
        }

        std::char::from_u32(shifted_ascii as u32).unwrap()
    } else {
        letter
    }
}

pub(crate) fn mel(line: &str, initial_shift: i32, encrypt: bool) -> String {
    let mut converted = String::new();
    let shift = if encrypt { initial_shift } else { -initial_shift };

    for letter in line.chars() {
        let letter_no_accent = replace_accented_char(letter);
        if letter_no_accent.is_alphabetic() {
            converted.push(shift_char(letter_no_accent, shift));
        } else {
            converted.push(letter);
        }
    }

    converted
}
