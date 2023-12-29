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

pub(crate) fn mel(line: &str, shift: i32) -> String {
    let mut converted = String::new();

    for letter in line.chars() {
        converted.push(shift_char(letter, shift));
    }

    converted
}

#[test]
fn test_rotate() {
    assert_eq!(mel("abc", 3), "def");
    assert_eq!(mel("xyz", 3), "abc");
    assert_eq!(mel("Hello World!", 13), "Uryyb Jbeyq!");
}