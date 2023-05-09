use std::str;

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.trim().len() <= 1 {
        return false;
    }

    if code.bytes().any(|b| !(b.is_ascii_digit() || b.is_ascii_whitespace())) {
        return false;
    }

    code.bytes()
        .filter(|c| *c >= b'0')
        .map(|c| (c - b'0') as u32)
        .rev()
        .enumerate()
        .map(|(i, number)| match number {
            number if i % 2 == 1 && number < 5 => number * 2,
            number if i % 2 == 1 => number * 2 - 9,
            _ => number,
        })
        .map(|e| {
            dbg!(e);
            e
        })
        .sum::<u32>()
        % 10
        == 0
}
