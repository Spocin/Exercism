use regex::Regex;

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    //Spaces are allowed but should be stripped
    let code_sanitized = code.replace(" ", "");

    //Strings of length 1 or less are not valid
    if code_sanitized.len() < 2 {
        return false;
    }

    //Only numbers are allowed
    let only_numbers_pattern = "^[0-9]*$";
    let regex_matcher = Regex::new(only_numbers_pattern).unwrap();

    if !regex_matcher.is_match(&code_sanitized) {
        return false;
    }

    //1. Double every second digit
    let step_one = code_sanitized
        .chars()
        .rev()
        .map(|digit| {
            digit.to_digit(10).unwrap()
        })
        .collect::<Vec<u32>>();

    let step_one_multiplied = step_one
        .iter()
        .enumerate()
        .map(|(idx, val)| {
            if idx % 2 != 0 {
                let val_multiplied = val * 2;
                if val_multiplied >= 10 {
                    return val_multiplied - 9;
                }
                return val_multiplied;
            }

            return *val;
        })
        .collect::<Vec<u32>>();

    //2. Sum all the digits
    let sum: u32 = step_one_multiplied.iter().sum();

    //3. Is sum divisible  by 10
    if sum % 10 != 0 {
        return false;
    }

    return true;
}
