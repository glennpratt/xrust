pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut len = 0;
    let x = isbn
        .chars()
        .filter_map(isbn_digit)
        .enumerate()
        .fold(0, |acc, (i, d)| {
            len = len + 1;
            if i < 9 && d > 9 {
                len = 100; // this is crap
            }
            acc + (d * (10 - i as u32))
        },
    );

    len == 10 && x % 11 == 0
}

fn isbn_digit(c: char) -> Option<u32> {
    match c {
        'x' | 'X' => Some(10),
        _ => c.to_digit(10),
    }
}
