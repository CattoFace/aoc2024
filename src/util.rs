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

pub fn fast_parse_backwards<T>(input: &[u8]) -> (T, usize)
where
    T: std::ops::Add<Output = T> + std::ops::Mul<Output = T> + From<u8> + Clone + std::marker::Copy,
{
    let mut sum = T::from(0u8);
    let mut ten_power: T = T::from(1u8);
    let ten: T = T::from(10u8);
    for (i, &c) in input.iter().rev().enumerate() {
        if c.is_ascii_digit() {
            sum = sum + T::from(c - b'0') * ten_power;
            ten_power = ten_power * ten;
        } else {
            return (sum, i);
        }
    }
    (sum, input.len() - 1)
}

pub fn fast_parsei<T>(input: &[u8]) -> (T, &[u8])
where
    T: std::ops::Add<Output = T> + std::ops::Mul<Output = T> + From<i8> + Clone + std::marker::Copy,
{
    let mut remainder = input;
    let mut sum = T::from(0i8);
    let ten: T = T::from(10i8);
    let negative_mul = if input[0] == b'-' {
        remainder = &remainder[1..];
        T::from(-1)
    } else {
        T::from(1)
    };
    while !remainder.is_empty() && remainder[0] >= b'0' && remainder[0] <= b'9' {
        sum = sum * ten + T::from((remainder[0] - b'0') as i8);
        remainder = &remainder[1..];
    }

    (sum * negative_mul, remainder)
}

#[derive(Debug)]
pub struct MyBucketQueue<T> {
    start: usize,
    data: Vec<Vec<T>>,
}

impl<T> MyBucketQueue<T> {
    pub fn new() -> MyBucketQueue<T> {
        MyBucketQueue {
            data: Default::default(),
            start: 0,
        }
    }
    pub fn with_capacity(capcity: usize) -> MyBucketQueue<T> {
        MyBucketQueue {
            data: Vec::with_capacity(capcity),
            start: 0,
        }
    }

    pub fn push(&mut self, key: usize, value: T) {
        if key >= self.data.len() {
            let missing = key - self.data.len() + 1;
            self.data.reserve(missing);
            for _ in 0..missing {
                self.data.push(Default::default());
            }
        }
        self.start = self.start.min(key);
        self.data[key].push(value);
    }

    pub fn pop(&mut self) -> Option<(usize, T)> {
        if self.data[self.start].is_empty() {
            self.start += self.data[self.start..].iter().position(|v| !v.is_empty())?;
        }
        let value = self.data[self.start].pop()?;
        Some((self.start, value))
    }
}

impl<T> Default for MyBucketQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}
