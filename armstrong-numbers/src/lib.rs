pub fn is_armstrong_number(value: u32) -> bool {
    let mut num = value;
    let mut digit_count: u32 = 0;
    while num > 0 {
        num /= 10;
        digit_count += 1;
    }
    let mut num = value;
    let mut acc: u128 = 0;
    while num > 0 {
        let digit:u128 = num as u128 % 10;
        num /= 10;
        acc += digit.pow(digit_count);
    }
    acc == value.into()
}
