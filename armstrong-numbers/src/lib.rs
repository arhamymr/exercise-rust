pub fn is_armstrong_number(num: u32) -> bool {
    // properly hanldles overflow
    if num >= 3_999_999_999 {
        return false;
    }

    // count digit number
    let number_of_digit = num.to_string().len() as u32;

    // sum of each digit to the power of number_of_digit
    let sum = num
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap().pow(number_of_digit))
        .sum::<u32>();

    // return true if sum is equal to num
    sum == num
}
