/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    println!("{code}");
    let mut trimmed_code: String = code.chars().filter(|ch| *ch != ' ').collect::<String>();
    if trimmed_code.len() < 2 {
        return false;
    }
    trimmed_code = trimmed_code.trim_start_matches('0').to_string();
    if trimmed_code.chars().filter(|ch| !ch.is_digit(10)).count() > 0 {
        return false;
    }

    trimmed_code
        .chars()
        .rev()
        .filter(|ch| ch.is_numeric())
        .map(|ch| ch.to_digit(10).unwrap())
        .enumerate()
        .map(|(index, digit)| match (index % 2) == 0 {
            true => digit,
            false => {
                if (digit * 2) > 9 {
                    digit * 2 - 9
                } else {
                    digit * 2
                }
            }
        })
        .sum::<u32>()
        % 10
        == 0
}
