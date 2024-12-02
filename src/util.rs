pub fn fast_parse<T>(input: &[u8]) -> (T, &[u8])
where
    T: std::ops::Add<Output = T> + std::ops::Mul<Output = T> + From<u8> + Clone + std::marker::Copy,
{
    let mut remainder = input;
    let mut sum = T::from(0u8);
    let ten: T = T::from(10u8);
    while !remainder.is_empty() && remainder[0] >= b'0' && remainder[0] <= b'9' {
        sum = sum * ten + T::from(remainder[0] - b'0');
        remainder = &remainder[1..];
    }
    (sum, remainder)
}

#[allow(dead_code)]
pub fn fast_parsei<T>(input: &[u8]) -> (T, &[u8])
where
    T: std::ops::Add<Output = T> + std::ops::Mul<Output = T> + From<i8> + Clone + std::marker::Copy,
{
    let mut remainder = input;
    let mut sum = T::from(0i8);
    let ten: T = T::from(10i8);
    while !remainder.is_empty() && remainder[0] >= b'0' && remainder[0] <= b'9' {
        sum = sum * ten + T::from((remainder[0] - b'0') as i8);
        remainder = &remainder[1..];
    }
    (sum, remainder)
}
