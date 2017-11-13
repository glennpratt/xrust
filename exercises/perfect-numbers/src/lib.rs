#[derive(Debug, PartialEq)]
pub enum Classification {
    Abundant,
    Deficient,
    Perfect,
}

pub fn classify(n: u64) -> Result<Classification, &'static str> {
    if n == 0 {
        return Err("Number must be positive");
    }
    let s = sum_of_factors(n);
    Ok(if n == s {
        Classification::Perfect
    } else if n < s {
        Classification::Abundant
    } else {
        Classification::Deficient
    })
}


fn sum_of_factors(n: u64) -> u64 {
    (1..n)
        .filter_map(|x| if n % x == 0 { Some(x) } else { None })
        .sum()
}
