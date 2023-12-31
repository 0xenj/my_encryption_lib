/// Replaces accented characters with their non-accented equivalents.
/// 
/// This function is particularly useful for text processing where accented characters
/// need to be considered as their basic letter equivalents.
///
/// # Arguments
/// * `c` - The character to be replaced if it is accented.
///
/// # Returns
/// Returns the non-accented equivalent of the character if it is accented.
/// Otherwise, returns the character itself.
pub(crate) fn replace_accented_char(c: char) -> char {
    match c {
        // Mapping for lowercase accented characters to their non-accented equivalents.
        'á' | 'à' | 'â' | 'ä' | 'ã' | 'å' | 'ā' => 'a',
        'é' | 'è' | 'ê' | 'ë' | 'ē' | 'ė' | 'ę' => 'e',
        'í' | 'ì' | 'î' | 'ï' | 'ī' | 'į' => 'i',
        'ó' | 'ò' | 'ô' | 'ö' | 'õ' | 'ø' | 'ō' => 'o',
        'ú' | 'ù' | 'û' | 'ü' | 'ū' => 'u',

        // Mapping for uppercase accented characters to their non-accented equivalents.
        'Á' | 'À' | 'Â' | 'Ä' | 'Ã' | 'Å' | 'Ā' => 'A',
        'É' | 'È' | 'Ê' | 'Ë' | 'Ē' | 'Ė' | 'Ę' => 'E',
        'Í' | 'Ì' | 'Î' | 'Ï' | 'Ī' | 'Į' => 'I',
        'Ó' | 'Ò' | 'Ô' | 'Ö' | 'Õ' | 'Ø' | 'Ō' => 'O',
        'Ú' | 'Ù' | 'Û' | 'Ü' | 'Ū' => 'U',

        // Return the character as is if it's not in the above list.
        _ => c,
    }
}
