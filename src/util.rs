pub fn fast_parse<T>(mut input: &[u8]) -> (T, &[u8])
where
    T: std::ops::Add<Output = T> + std::ops::Mul<Output = T> + From<u8> + Clone + std::marker::Copy,
{
    let mut sum = T::from(0u8);
    let ten: T = T::from(10u8);
    while !input.is_empty() && input[0] >= b'0' && input[0] <= b'9' {
        sum = sum * ten + T::from(input[0] - b'0');
        input = &input[1..];
    }
    (sum, input)
}
